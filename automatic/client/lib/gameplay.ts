import { down, moveTo, up } from "./input";
import { solve, stringifyRawBoard } from "./solver";
import { type Game, GameState, type Sprite, type SwapLetterButton, type Vec2 } from "./types/extern";
import { UI } from "./ui";
import { awaitWrapper, waitForValue } from "./utils";

/**
 * Traverses sprite's parents to determine it's position.
 * For whatever reason we do not need to apply scale of stage (it is the root parent of everything else).
 * @param sprite Sprite itself
 * @returns Coordinates relative to game canvas
 */
function getSpriteCoords(sprite: Sprite): Vec2 {
  let x = sprite.position.x;
  let y = sprite.position.y;
  let parent = sprite.parent;
  while (parent) {
    if (parent == sprite.stage) break;
    x = parent.position.x + x * parent.scale.x;
    y = parent.position.y + y * parent.scale.y;
    parent = parent.parent;
  }
  return { x, y };
}

/**
 * Waits for a sprite to appear.
 * @param root A root sprite children of which may contain the needed sprite
 * @param predicate Predicate that checks whether sprite is one we're looking for
 * @param interval Interval passed to waitForValue
 * @returns Promise, that resolves with sprite when it's found
 */
function waitForSprite(
  root: Sprite,
  predicate: (x: Sprite) => boolean,
  interval: number
): [Promise<Sprite>, () => void] {
  function recursion(parent: Sprite, limit: number = 5): Sprite | void {
    if (limit <= 0) return;
    for (let child of parent.children) {
      if (predicate(child)) return child;
      let result = recursion(child, limit - 1);
      if (result) return result;
    }
  }
  return waitForValue(() => recursion(root), interval);
}

const GAMEPLAY = new (class GlobalGameplay {
  canvas: HTMLCanvasElement;
  game: Game;
  isBusy: boolean;
  watchdog: number | null;

  constructor() {
    let canvas = document.querySelector("canvas#gameCanvas") as HTMLCanvasElement;
    if (!canvas) throw new Error("Failed to get game canvas");
    this.canvas = canvas;
    // This is safe, since all usages of this.game happen after handleHook sets this.game to actual Game.
    this.game = {} as Game;
    this.isBusy = false;
    this.watchdog = null;
  }

  moveToSprite(sprite: Sprite) {
    let { x, y } = getSpriteCoords(sprite);
    moveTo(this.canvas, x, y);
  }

  getTileSprite(index: number): Sprite {
    let tile = Object.values(this.game.board.letterPieces).find(
      (x) => x.letterData.row * 5 + x.letterData.collumn == index
    );
    if (!tile) {
      throw new Error(`Failed to get sprite for tile ${index}`);
    }
    return tile;
  }

  clickSprite(sprite: Sprite) {
    this.moveToSprite(sprite);
    down(this.canvas);
    up(this.canvas);
  }

  async makeSwap(index: number, letter: string) {
    let message = `Swapping tile ${index} to ${letter}...`;
    this.clickSprite(this.game.spellbook.powerupButtons.CHANGE);
    let tile = this.getTileSprite(index);
    // Apparently tile.alpha becomes 0 when it starts shaking and goes back to 1 only when swap is complete.
    await awaitWrapper(
      waitForValue(() => tile.alpha == 0, 10),
      message
    );
    this.clickSprite(tile);
    let parent = this.game.parent?.parent;
    // It should never happen, so if it does, let's just throw the error.
    if (!parent) throw new Error("Failed to get game.parent.parent");
    this.clickSprite(
      await awaitWrapper(
        waitForSprite(parent, (x) => (x as SwapLetterButton)?.config?.key == letter, 25),
        message
      )
    );
    await awaitWrapper(
      waitForValue(() => tile.alpha == 1, 10),
      message
    );
  }

  async play() {
    if (!this.game.isMyTurn) return UI.showStatus("Not our turn");
    if (this.isBusy) return;
    if (!this.watchdog) this.setupWatchdog();
    this.isBusy = true;
    // First isMyTurn=true in  usually happens before board is ready.
    await awaitWrapper(
      waitForValue(() => Object.values(this.game.board.boardData.letters).length == 25, 10),
      "Waiting for the board..."
    );
    // Just in case board scale animation is still playing.
    let waitMaybe = waitForValue(() => !this.game.board.isLocked, 10);
    let swaps = Math.floor(this.game.spellbook.manaCounter.manaCount / 3);
    let result = await awaitWrapper(
      solve(stringifyRawBoard(this.game.board.boardData), swaps, swaps > 0 ? UI.getThreads() : 1),
      "Solving the board..."
    );
    await awaitWrapper(waitMaybe, "Waiting for board to unlock...");
    let best = result.words[0];
    if (!best) {
      // TODO: Add ability to retry. Or board shuffle.
      throw new Error("No solution found");
    }
    for (let step of best.steps) {
      if (step.swap && step.new_letter) {
        await this.makeSwap(step.index, step.new_letter.toUpperCase());
      }
    }
    for (let _index in best.steps) {
      // Thanks JavaScript for stupid string array index.
      let index = Number(_index);
      let step = best.steps[index];
      this.moveToSprite(this.getTileSprite(step.index));
      if (index == 0) {
        down(this.canvas);
      } else if (index == best.steps.length - 1) {
        up(this.canvas);
      }
    }
    this.isBusy = false;
  }

  errorHandler(e: any) {
    console.error(e);
    let string = e + "";
    UI.showOverlay(
      `${string.length < 75 ? string : string.slice(0, 75) + "..."} (check the console)`,
      () => {
        this.isBusy = false;
        UI.hideOverlay();
        UI.showStatus("Trying to recover...");
        this.handleCurrentState();
      },
      "Recover?"
    );
  }

  setupWatchdog() {
    this.watchdog = window.setInterval(() => {
      if (!this.isBusy && this.game.isMyTurn && !this.game.board.isLocked) {
        this.handleCurrentState();
      }
    }, 500);
  }

  handleCurrentState() {
    switch (this.game.currentGameState) {
      case GameState.LOBBY:
        return UI.showStatus("Idle");
      case GameState.PLAYING:
        return this.play().catch((e) => this.errorHandler(e));
      case GameState.GAME_OVER:
        return UI.showStatus("GG!");
      default:
        return UI.showStatus("How Did We Get Here?");
    }
  }

  handleHook(game: Game) {
    this.game = game;
    UI.hideOverlay();
    this.handleCurrentState();
    // @ts-ignore
    unsafeWindow.WSdebug = {
      game,
      gg: this,
      ui: UI,
    };
  }
})();

function hookCallback(this: Game, isMyTurn: boolean) {
  Object.defineProperty(this, "isMyTurn", {
    get() {
      return isMyTurn;
    },
    configurable: true,
    set: hookCallback,
  });
  if (typeof this.spellbook !== "undefined") {
    GAMEPLAY.handleHook(this);
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

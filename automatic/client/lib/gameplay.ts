import { down, moveTo, up } from "./input";
import { solve, stringifyRawBoard } from "./solver";
import { type Game, GameState, type Sprite, type SwapLetterButton, type Vec2 } from "./types/extern";
import { UI } from "./ui";
import { awaitWrapper, sleep, waitForValue } from "./utils";

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

  constructor() {
    // This is safe, since all usages of this.game happen after handleHook sets this.game to actual Game.
    let canvas = document.querySelector("canvas#gameCanvas") as HTMLCanvasElement;
    if (!canvas) throw new Error("Failed to get game canvas");
    this.canvas = canvas;
    this.game = {} as Game;
    this.isBusy = false;
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
    this.clickSprite(this.game.spellbook.powerupButtons.CHANGE);
    await awaitWrapper(sleep(100));
    let tile = this.getTileSprite(index);
    this.clickSprite(tile);
    let parent = this.game.parent?.parent;
    // It should never happen, so if it does, let's just throw the error.
    if (!parent) throw new Error("Failed to get game.parent.parent");
    this.clickSprite(
      await awaitWrapper(waitForSprite(parent, (x) => (x as SwapLetterButton)?.config?.key == letter, 25))
    );
    // Apparently tile.alpha becomes 0 when it starts shaking and goes back to 1 only when swap is complete.
    await awaitWrapper(waitForValue(() => tile.alpha == 1, 10));
  }

  // TODO: Add error handler for all the stuff that happens in play() (it's mostly interrupt errors).
  async play(isMyTurn: boolean) {
    if (!isMyTurn) return;
    this.isBusy = true;
    // First isMyTurn=true in  usually happens before board is ready.
    await awaitWrapper(waitForValue(() => Object.values(this.game.board.boardData.letters).length == 25, 10));
    // Just in case board scale animation is still playing.
    let sleepMaybe = sleep(200);
    let result = await awaitWrapper(
      solve(stringifyRawBoard(this.game.board.boardData), Math.floor(this.game.spellbook.manaCounter.manaCount / 3), 12)
    );
    // Not awaiting it immediately, since time may have already been passed while solver was running.
    await awaitWrapper(sleepMaybe);
    let best = result.words[0];
    if (!best) {
      // TODO: Add ability to retry. Or board shuffle.
      return console.error("No solution found");
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

  handleHook(game: Game, isMyTurn: boolean) {
    this.game = game;
    //@ts-ignore
    unsafeWindow._game = this.game;
    //@ts-ignore
    unsafeWindow._gg = this;
    UI.hideOverlay();
    if (this.isBusy) return;
    switch (game.currentGameState) {
      case GameState.MENU:
        return console.log("In menus");
      case GameState.GAME:
        return this.play(isMyTurn);
      case GameState.GAMEOVER:
        return console.log("GG!");
    }
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
    GAMEPLAY.handleHook(this, isMyTurn);
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

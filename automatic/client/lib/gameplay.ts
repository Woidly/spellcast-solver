import { down, moveTo, up } from "./input";
import { solve, stringifyRawBoard } from "./solver";
import { type Game, GameState, type Sprite, type Vec2 } from "./types/extern";
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

  // TODO: Add error handler for all the stuff that happens in play() (it's mostly interrupt errors).
  async play(isMyTurn: boolean) {
    if (!isMyTurn) return;
    this.isBusy = true;
    // First isMyTurn=true in  usually happens before board is ready.
    await awaitWrapper(waitForValue(() => Object.values(this.game.board.boardData.letters).length == 25));
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
    for (let _index in best.steps) {
      // Thanks JavaScript for stupid string array index.
      let index = Number(_index);
      let step = best.steps[index];
      if (step.swap) {
        console.error("Swap moves aren't implemented yet"); // TODO
      }
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

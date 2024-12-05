import { down, moveTo, up } from "./mouse";
import { solve, stringifyRawBoard } from "./solver";
import type { Game, Sprite, SwapLetterButton, Vec2 } from "./types/extern";
import type { Move } from "./types/solver";
import { UI } from "./ui";
import { sleep } from "./utils";

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
 * @returns Promise, that resolves with sprite when it's found
 */
function waitForSprite(root: Sprite, predicate: (x: Sprite) => boolean): Promise<Sprite> {
  let resolve: (value: Sprite) => void;
  let promise: Promise<Sprite> = new Promise((_resolve) => (resolve = _resolve));
  function recursion(parent: Sprite, limit: number = 5): Sprite | void {
    if (limit <= 0) return;
    for (let child of parent.children) {
      if (predicate(child)) return child;
      let result = recursion(child, limit - 1);
      if (result) return result;
    }
  }
  // TODO: Make it interruptable. Maybe even implement some global cleanup hook so there aren't any dangling intervals.
  let interval = setInterval(() => {
    let found = recursion(root);
    if (found) {
      resolve(found);
      clearInterval(interval);
    }
  }, 50);
  return promise;
}

function filterSwaps(moves: Move[]): [number, string][] {
  let swaps: [number, string][] = [];
  for (let move of moves) {
    if (move.swap && move.new_letter) {
      swaps.push([move.index, move.new_letter.toUpperCase()]);
    }
  }
  return swaps;
}

const GAMEPLAY = new (class GlobalGameplay {
  game: Game;
  canvas: HTMLCanvasElement;

  constructor() {
    // Just putting it here to please TypeScript.
    // It won't cause any issues as any interactions with this.game are called after handleHook that sets up game properly.
    this.game = {} as Game;
    let canvas = document.querySelector("canvas#gameCanvas") as HTMLCanvasElement;
    if (!canvas) {
      // FIXME: More graceful shutdown, maybe let user retry it.
      UI.showOverlay("Game canvas not found");
      throw new Error("Game canvas not found");
    }
    this.canvas = canvas;
    //
  }

  getBoard(): string | null {
    if (Object.values(this.game.board.boardData.letters || []).length != 25) return null;
    return stringifyRawBoard(this.game.board.boardData);
  }

  getSwaps(): number {
    // Using function instead of just having this expression as later we may want to not use all swaps to save them for later (gem management)
    return Math.floor(this.game.spellbook.manaCounter.manaCount / 3);
  }

  getGemValue(): number {
    return 0; // TODO
  }

  moveToSprite(sprite: Sprite) {
    let { x, y } = getSpriteCoords(sprite);
    moveTo(this.canvas, x, y);
  }

  clickSprite(sprite: Sprite) {
    this.moveToSprite(sprite);
    down(this.canvas);
    up(this.canvas);
  }

  async getTile(index: number): Promise<Sprite> {
    let tile = Object.values(this.game.board.letterPieces).find(
      (x) => x.letterData.row * 5 + x.letterData.collumn == index
    );
    if (!tile) {
      throw new Error(`Failed to get sprite for tile ${index}`);
    }
    return tile;
  }

  async makeSwap(index: number, letter: string) {
    this.clickSprite(this.game.spellbook.powerupButtons.CHANGE);
    await new Promise((r) => setTimeout(r, 100));
    this.clickSprite(await this.getTile(index));
    let parent = this.game.parent?.parent;
    // It should never happen, so if it does, let's just throw the error.
    if (!parent) throw new Error("Failed to get game.parent.parent");
    let letterButton = await waitForSprite(parent, (x) => (x as SwapLetterButton)?.config?.key == letter);
    this.clickSprite(letterButton);
    // Too fast. Too soon.
    await sleep(1500);
  }

  async play() {
    let board;
    if (!(board = this.getBoard())) return UI.showOverlay("Board not found!");
    UI.showOverlay("Solving board...");
    let results;
    try {
      // TODO: Make it interruptable.
      results = await solve(board, this.getSwaps(), this.getGemValue());
    } catch (e) {
      console.error(e); // TODO: Show errors directly in the UI.
      return UI.showOverlay("Solver error");
    }

    let best = results.solutions[0];
    if (!best) {
      return UI.showOverlay("No solution found");
    }
    // UI.log(`${best.word} +${best.score}`);

    // Just in case board scale animation is still playing.
    await sleep(150);
    let swaps;
    if ((swaps = filterSwaps(best.moves))) {
      // UI.hideOverlay();
      // UI.log("Swapping letter...")
      UI.showOverlay("Swapping letters...");
      for (let [index, letter] of swaps) {
        await this.makeSwap(index, letter);
      }
      UI.hideOverlay();
    }
    UI.showOverlay("Making move...");
    for (let _index in best.moves) {
      // Thanks javascript.
      let index = Number(_index);
      let move = best.moves[index];
      this.moveToSprite(await this.getTile(move.index));
      if (index == 0) {
        down(this.canvas);
      } else if (index == best.moves.length - 1) {
        up(this.canvas);
      }
    }
    UI.hideOverlay();
    // UI.log(`${best.word} +${best.score}`);
  }

  handleHook(game: Game, isMyTurn: boolean) {
    this.game = game;
    if (isMyTurn) {
      this.play();
    } else {
      UI.showOverlay("Not our turn");
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
    // FIXME: Do something about timeout.
    // Initially it was added to avoid blocking setter (but later function became async and there's no more need for it).
    // However, if we remove it, neither of two isMyTurn=true assigns would have a board in place.
    // Setting it to 1ms seems to work, as first assign still has no board and second one has just enough time to have a board.
    // However, it may cause issues because of lags.
    // Probably smartest decision would be to make waitFor interval thingy, but then two calls will overlap and both have a board.
    // I'll figure it out later.
    setTimeout(() => GAMEPLAY.handleHook(this, isMyTurn), 1);
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

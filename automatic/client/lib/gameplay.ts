import { down, moveTo, up } from "./mouse";
import { solve, stringifyRawBoard } from "./solver";
import type { Game, Sprite, SwapLetterButton, Vec2 } from "./types/extern";

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
  let interval = setInterval(() => {
    let found = recursion(root);
    if (found) {
      resolve(found);
      clearInterval(interval);
    }
  }, 50);
  return promise;
}

async function play(game: Game) {
  if (Object.values(game.board.boardData.letters || []).length != 25) return;
  let board = stringifyRawBoard(game.board.boardData);
  let swaps = Math.floor(game.spellbook.manaCounter.manaCount / 3);
  let gem_value = 0; // TODO: Somehow implement gem management.
  let results;
  try {
    results = await solve(board, swaps, gem_value);
  } catch (e) {
    // TODO: Implement error handling, probably print errors to UI, as console.log/warn/error/debug/etc is patched by Discord.
    throw e;
  }
  let best = results.solutions[0];
  if (!best) {
    return console.error("No solution found");
  }
  console.log(`${best.word} +${best.score}`);
  // Wait for board scale animation to finish
  // TODO: Maybe skip animation entirely, we literally can control the game.
  await new Promise((r) => setTimeout(r, 300));
  let canvas = document.querySelector("canvas#gameCanvas");
  if (!canvas) {
    return console.error("Failed to get the game canvas");
  }
  for (let move of best.moves) {
    if (move.swap) {
      let { x, y } = getSpriteCoords(game.spellbook.powerupButtons.CHANGE);
      moveTo(canvas, x, y);
      down(canvas);
      up(canvas);
      let sprite = Object.values(game.board.letterPieces).find(
        (x) => x.letterData.row * 5 + x.letterData.collumn == move.index
      );
      if (!sprite) {
        return console.error(`Failed to get sprite for tile ${move.index}`);
      }
      ({ x, y } = getSpriteCoords(sprite));
      await new Promise((r) => setTimeout(r, 100));
      moveTo(canvas, x, y);
      down(canvas);
      up(canvas);
      let letterButton = await waitForSprite(game.parent?.parent || game, (x) => {
        return (x as SwapLetterButton)?.config?.key == move.new_letter?.toUpperCase();
      });
      ({ x, y } = getSpriteCoords(letterButton));
      moveTo(canvas, x, y);
      down(canvas);
      up(canvas);
      // Too fast. Too soon.
      await new Promise((r) => setTimeout(r, 1500));
    }
  }
  for (let _index in best.moves) {
    // Thanks javascript.
    let index = Number(_index);
    let move = best.moves[index];
    let sprite = Object.values(game.board.letterPieces).find(
      (x) => x.letterData.row * 5 + x.letterData.collumn == move.index
    );
    if (!sprite) {
      return console.error(`Failed to get sprite for tile ${move.index}`);
    }
    let { x, y } = getSpriteCoords(sprite);
    moveTo(canvas, x, y);
    if (index == 0) {
      down(canvas);
    } else if (index == best.moves.length - 1) {
      up(canvas);
    }
  }
}

function hookCallback(this: Game, isMyTurn: boolean) {
  Object.defineProperty(this, "isMyTurn", {
    get() {
      return isMyTurn;
    },
    configurable: true,
    set: hookCallback,
  });
  if (isMyTurn && typeof this.spellbook !== "undefined") {
    setTimeout(() => play(this), 1);
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

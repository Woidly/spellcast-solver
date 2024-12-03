import { solve, stringifyRawBoard } from "./solver";
import type { Game } from "./types/extern";

async function play(game: Game) {
  let board = stringifyRawBoard(game.board.boardData);
  let swaps = Math.floor(game.spellbook.manaCounter.manaCount / 3);
  let gem_value = 0; // TODO: Somehow implement gem management.
  let results;
  try {
    results = solve(board, swaps, gem_value);
  } catch (e) {
    // TODO: Implement error handling, probably print errors to UI, as console.log/warn/error/debug/etc is patched by Discord.
    throw e;
  }
  console.log(results);
  // TODO: Play the moves.
}

function hookCallback(this: Game, isMyTurn: boolean) {
  Object.defineProperty(this, "isMyTurn", { value: isMyTurn, configurable: true, set: hookCallback });
  if (isMyTurn) {
    setTimeout(() => play(this), 1);
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

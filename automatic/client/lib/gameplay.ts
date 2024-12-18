import { solve, stringifyRawBoard } from "./solver";
import type { Game } from "./types/extern";

function hookCallback(this: Game, isMyTurn: boolean) {
  Object.defineProperty(this, "isMyTurn", {
    get() {
      return isMyTurn;
    },
    configurable: true,
    set: hookCallback,
  });
  if (typeof this.spellbook !== "undefined") {
    //@ts-ignore
    unsafeWindow._game = this;
    console.log("isMyTurn updated:", isMyTurn);
    if (isMyTurn) {
      setTimeout(() => {
        let [promise, _] = solve(
          stringifyRawBoard(this.board.boardData),
          Math.floor(this.spellbook.manaCounter.manaCount / 3),
          12
        );
        promise.then((x) => console.log("Solved", x)).catch((e) => console.error("Solver error", e));
      }, 500);
    }
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

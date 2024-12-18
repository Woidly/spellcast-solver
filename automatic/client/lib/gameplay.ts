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
  }
}

export function hookGame() {
  Object.defineProperty(Object.prototype, "isMyTurn", {
    configurable: true,
    set: hookCallback,
  });
}

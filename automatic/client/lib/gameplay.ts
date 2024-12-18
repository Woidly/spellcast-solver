import { solve, stringifyRawBoard } from "./solver";
import { type Game, GameState } from "./types/extern";

const GAMEPLAY = new (class GlobalGameplay {
  game: Game;

  constructor() {
    // This is safe, since all usages of this.game happen after handleHook sets this.game to actual Game.
    this.game = {} as Game;
  }

  play(isMyTurn: boolean) {
    if (isMyTurn) {
      setTimeout(() => {
        let [promise, _] = solve(
          stringifyRawBoard(this.game.board.boardData),
          Math.floor(this.game.spellbook.manaCounter.manaCount / 3),
          12
        );
        promise.then((x) => console.log("Solved", x)).catch((e) => console.error("Solver error", e));
      }, 500);
    }
  }

  handleHook(game: Game, isMyTurn: boolean) {
    this.game = game;
    //@ts-ignore
    unsafeWindow._game = this.game;
    //@ts-ignore
    unsafeWindow._gg = this;
    console.log("isMyTurn updated:", isMyTurn);
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

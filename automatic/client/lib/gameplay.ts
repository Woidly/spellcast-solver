import { solve, stringifyRawBoard } from "./solver";
import { type Game, GameState } from "./types/extern";
import { awaitWrapper, sleep, waitForValue } from "./utils";

const GAMEPLAY = new (class GlobalGameplay {
  game: Game;
  isBusy: boolean;

  constructor() {
    // This is safe, since all usages of this.game happen after handleHook sets this.game to actual Game.
    this.game = {} as Game;
    this.isBusy = false;
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
    console.log("Solved", result);
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

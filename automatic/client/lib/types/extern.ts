/**
 * All the types are from Spellcast itself.
 * Nothing here is written by me, it's just a type definitions.
 * Properties that I do not use aren't defined.
 * Most of type names are made up.
 */

/** Just a usual X/Y Vec2 */
export type Vec2 = {
  x: number;
  y: number;
};

/** Base type for every game object */
export type Sprite = {
  /** Position of this sprite's centre relative to its parent */
  position: Vec2;
  /** Scale of this sprite */
  scale: Vec2;
  /** Child sprites */
  children: Sprite[];
  /** Parent sprite */
  parent: Sprite | null;
  /** Stage (basically a sprite as big as game canvas, parent of all other sprites) */
  stage: Sprite;
  /**
   * Whether the sprite is "visible".
   * Even with this set to true, sprite may be invisible due to other properties like `worldAlpha`.
   */
  visible: boolean;
  /**
   * Global alpha (basically opacity of this sprite).
   * Even with this set to 1, sprite may be invisible due to other properties like `visible`.
   */
  worldAlpha: number;
};

/** Sprites that let  */
export type SwapLetterButton = Sprite & {
  config: {
    /** Actual letter on the tile, uppercase */
    key: string;
  };
};

/** Tile position on board */
export type TilePos = {
  /** Column tile is located in, 0-4 inclusive (devs made a typo lol) */
  collumn: number;
  /** Row tile is located in, 0-4 inclusive */
  row: number;
};

export type TileData = TilePos & {
  /** Whether the tile is frozen */
  block_letter: boolean;
  /** Actual letter on the tile, uppercase */
  key: string;
  /** Amount of gems tile has, 0-1 inclusive */
  letter_mana: number;
  /** @returns Letter multiplier of this tile, 1/2/3 */
  getLetterMultiplier(): number;
  // There's also getWordMultiplier(), but it seems to be broken.
  // Position of tile with word multiplier is stored in board.boardData.
};

export type BoardData = {
  /** Mapping of tile id to tile data */
  letters: Record<number, TileData>;
  /** Position of tile with 2x multiplier */
  wordMultiplierPosition: TilePos | null;
};

export enum GameState {
  MENU = 1,
  GAME = 2,
  GAMEOVER = 3,
}

/** This single object carries the whole thing */
export type Game = Sprite & {
  /** Board sprite */
  board: {
    boardData: BoardData;
    /** Mapping of tile id to tile sprite */
    letterPieces: Record<
      number,
      Sprite & {
        letterData: TileData;
      }
    >;
  };
  /** Current game state */
  currentGameState: GameState;
  /** Whether it's currently our turn */
  isMyTurn: boolean;
  spellbook: {
    manaCounter: {
      /** Number of gems we currently have, 0-10 inclusive */
      manaCount: number;
    };
    powerupButtons: {
      /** Swap button sprite */
      CHANGE: Sprite;
    };
  };
  statsUi: {
    roundCounter: {
      /** Current round, 1-5 inclusive (or -1 when not in game) */
      currentRound: number;
    };
  };
};

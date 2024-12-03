/**
 * All the types are from Spellcast itself.
 * Nothing here is written by me, it's just a type definitions.
 * Properties that I do not use aren't defined.
 * Most of type names are made up.
 */

/**
 * Just a usual X/Y Vec2.
 * It's called Point in Spellcast, but this name fits its use more.
 */
export type Vec2 = {
  x: number;
  y: number;
};

/** Base type for everything in game */
export type Sprite = {
  /** Position of sprite's center relative to its parent */
  position: Vec2;
  /**
   * Sprite's scale.
   * Basically a multiplier for childrens' coordinates.
   */
  scale: Vec2;
  /** Child sprites */
  children: Sprite[];
  /** Parent sprite */
  parent: Sprite | null;
  /** Stage, basically a sprite as big as game canvas, parent of other sprites */
  stage: Sprite;
};

export type Tile = Sprite & {
  letterData: TileData;
};

export type TilePos = {
  /** Coulmn tile is located in, 0-4 inclusive (devs made a typo lol) */
  collumn: number;
  /** Row tils is located in, 0-4 inclusive */
  row: number;
};

/**
 * Not really a board.
 * Just tile data and word multiplier position.
 */
export type BoardGrid = {
  /**
   * Mapping of tile index to tile data.
   * Indexes are meaningless, just using it as list of letters.
   */
  letters: Record<number, TileData>;
  /**
   * Position of tile with 2x multiplier.
   * collumn and row both
   */
  wordMultiplierPosition: TilePos;
};

/** Information about a tile */
export type TileData = TilePos & {
  /** Actual letter on the tile, uppercase */
  key: string;
  /** Amount of gems tile has, 0-1 inclusive */
  letter_mana: number;
  /** @returns Letter multiplier of this tile, 1/2/3 */
  getLetterMultiplier(): number;

  // There's also getWordMultiplier(), but it seems to be broken.
  // Position of tile with word multiplier is stored in BoardState.
};

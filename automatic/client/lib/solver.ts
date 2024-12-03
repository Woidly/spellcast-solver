import { httpRequest } from "./http";
import type { BoardGrid, TileData } from "./types/extern";
import type { Results } from "./types/solver";

const SERVER = "http://localhost:27974/";

export function stringifyRawTile(raw: TileData, twox: boolean): string {
  return (
    raw.key +
    ({ 1: "", 2: "+", 3: "*" }[raw.getLetterMultiplier()] || "") +
    ({ 1: "", 2: "$", 3: "^" }[twox ? 2 : 1] || "") +
    (raw.letter_mana > 0 ? "!" : "")
  );
}

export function stringifyRawBoard(raw: BoardGrid): string {
  let board: (TileData | null)[][] = Array(5)
    .fill(null)
    .map((_) => Array(5).fill(null));
  for (let tile of Object.values(raw.letters)) {
    board[tile.row][tile.collumn] = tile;
  }
  return board
    .map((row) =>
      row
        .map((tile) => {
          if (!tile) {
            return ""; // Not going to happen, just added it to please TypeScript.
          }
          return stringifyRawTile(
            tile,
            tile.collumn == raw.wordMultiplierPosition?.collumn && tile.row == raw.wordMultiplierPosition?.row
          );
        })
        .join("")
    )
    .join("");
}

export function solve(board: string, swaps: number, gem_value: number = 0): Promise<Results> {
  let url = new URL(SERVER);
  url.searchParams.set("board", board);
  url.searchParams.set("swaps", swaps.toString());
  url.searchParams.set("gem_value", gem_value.toString());
  return httpRequest(url.toString(), "POST").then((text) => JSON.parse(text) as Results);
}

import { httpRequest } from "./http";
import type { BoardData, TileData } from "./types/extern";
import type { Results, ServerResponse } from "./types/solver";

const SERVER = "http://localhost:27974/";

function stringifyRawTile(raw: TileData, double: boolean): string {
  return (
    raw.key +
    ({ 1: "", 2: "+", 3: "*" }[raw.getLetterMultiplier()] || "") +
    (double ? "$" : "") +
    (raw.letter_mana > 0 ? "!" : "") +
    (raw.block_letter ? "#" : "")
  );
}

export function checkConnection(): Promise<string> {
  return httpRequest(SERVER, "GET")[0];
}

export function stringifyRawBoard(raw: BoardData): string {
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

export function solve(board: string, swaps: number, threads: number): [Promise<Results>, () => void] {
  let url = new URL(SERVER);
  url.searchParams.set("board", board);
  url.searchParams.set("swaps", swaps.toString());
  url.searchParams.set("threads", threads.toString());
  let [promise, interrupt] = httpRequest(url.toString(), "POST");
  return [
    promise.then((text) => {
      let r = JSON.parse(text) as ServerResponse;
      if (r.ok && r.data) {
        return r.data;
      } else {
        throw new Error(r.error);
      }
    }),
    interrupt,
  ];
}

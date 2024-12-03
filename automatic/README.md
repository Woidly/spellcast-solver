# automatic

This is sub-project written in TypeScript/JavaScript with Bun that allows to automatically play Spellcast using Rust solver.
It has two parts:

## Server

A server that allows to use Rust solver CLI via HTTP.
Server is powered by `Bun.serve`, therefore it doesn't require any dependencies and works out of the box.
To run it, just type `bun run server/index.ts` (server uses port `27974` by default).

## Client

A client that injects parts of it's code into the game, reads game state, uses the server to solve the board and makes the moves **completely automatically**.

(WIP)

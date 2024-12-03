# automatic

This is sub-project written in TypeScript with Bun that allows to automatically play Spellcast using Rust solver.
It has two parts:

## Server

A server that allows to use Rust solver CLI via HTTP.
Server is powered by `Bun.serve`, therefore it doesn't require any JS dependencies and works out of the box.

It needs paths to solver and dictionary to function properly, though.
If you run it from current directory like `bun run server/index.ts`, it should pick up `dictionary.txt` and `target/release/spellcast-solver` from repo's root automatically.

But if it does not (or you want to use custom paths), you can specify paths with `--dictionary` and `--solver` arguments (like `bun run server/index.ts --dictionary ~/dictionary.txt --solver /opt/spellcast-solver`).

By default server will start solver with 12 threads, you can change it via `THREADS` constant of [server/index.ts](server/index.ts#L48).
Default port for the server is `27974`.
There's no reason to change it, but if you do, don't forget to also change it in the client.

## Client

A client that injects parts of it's code into the game, reads game state, uses the server to solve the board and makes the moves **completely automatically**.

(WIP)

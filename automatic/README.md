# automatic

**The one and only Spellcast automation tool**

This is a userscript that allows [spellcast-solver](..) to automatically play best moves in Spellcast.
Because JavaScript or Tampermonkey can't run solver executable on your machine, it also requires a server.

This project is written in TypeScript with [Bun](https://bun.sh).

## Server

Server is powered by `Bun.serve`, therefore it doesn't require any JS dependencies and works out of the box.
Not much to say about it, it's very simple.

However, it needs paths to solver and dictionary.
If you run it from current directory like `bun run server`, it should pick up `dictionary.txt` and `target/release/spellcast-solver` from repo root automatically.
But if it doesn't (or you want to use custom paths), you can specify paths with `-d`/`--dictionary` and `-s`/`--solver` arguments (like `bun run server -d /usr/share/dictionary.txt -s /opt/spellcast-solver`).  
You can also change server port with `-p`/`--port`, however there's no reason to change it, as default port `27974` should certainly be free.
But if you do change port, don't forget to also change it in the client.

## Client (userscript)

WIP

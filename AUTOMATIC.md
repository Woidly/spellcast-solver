# Automatic solver setup guide (WIP)

This automatic solver is intended to be very lightweight, so it just reads pixels instead of more complicated stuff, so it is very sensitive to changes (and therefore requires very specific setup to work properly).
(Though, if you want, you can use OCR or AI and feed it's output into CLI solver.)
For the same reason it doesn't implement screenshot and mouse movement directly, leaving this up to you.

## Window size

To make it easier, you should probably use a proper browser instead of Electron-based Discord client, as you have access to DevTools by default.
Also, instead of opening activity inside of normal tab, use popout (a separate minimalistic browser window).
It has `https://discord.com/popout` in address bar and opens by default when you start Spellcast in a text channel.
After opening Spellcast window, open DevTools (**`Ctrl+Shift+I`** will work in most cases), make sure DevTools are in separate window too and locate a `<canvas id="gameCanvas" ... width="..." height="...">` element. Tip: use element selector (**`Ctrl+Shift+C`**) and click something in game, it should select canvas.
Resize the window until canvas is exactly 1280x720 (you can see width and height in inspect element).

In Brave (and, I suppose, any other Chromium-based browser) canvas has size of top window, except height is 48px smaller because of Discord UI, so you can use `window.resizeTo(1280, 720+48);` in console to quickly resize it. **(MAKE SURE `top` CONTEXT IS SELECTED IN CONSOLE!)**

After resizing the window and making sure canvas is properly sized, move it wherever you like (make sure it is fully visible, it will need to stay in this place for entire game).
I suggest avoiding places where a notification can pop up and keeping some screen space for a terminal.

## Window position

Now you need to calculate canvas offset.
Run the following code in console (**again, IN `top` CONTEXT**), it will create a make top-left pixel of canvas red:
```js
let r=document.querySelector("iframe").getBoundingClientRect(),e=document.createElement("div"),s=e.style;s.width=s.height="1px",s.backgroundColor="red",s.position="absolute",s.top=r.top+"px",s.left=r.left+"px",document.body.appendChild(e);
```
Now close DevTools and make a full-screen screenshot, open it in your favourite image editing program and locate this red pixel.
In my case, this pixel was at `22, 347`, remember it for later.
To avoid this calibration next time, make your window manager remember size and position of this window (for example, in Plasma there are "Window Rules").

## Screenshot

You'll need some command-line utility capable of making a full-screen screenshot and outputting it to file or solver's stdin via pipe.
In this example I'll be using KDE's Spectacle, but you can do it with almost any other tool you like (you can find more tools at https://wiki.archlinux.org/title/Screen_capture, many of those will work on non-Arch distros too).
AFAIK, Spectacle can't output screenshot into stdout, so I'll have to use a file.
And the final command will be `spectacle -fbno /tmp/spellshot.png`, where `f` - full-screen, `b` - background (no GUI), `n` - no notification, `o` - output path.

## Mouse

And the final part you'll need - mouse automation.
In solver's stdout (or output file) you'll find few commands separated by newlines.
There are only two commands, they are pretty simple to implement.

- `SWAP x1 y1 x2 y2 x3 y3` - swapping letter; you'll need to click swap button at `x1,y1`, wait for around 100 ms, click tile at `x2,y2`, wait for swap menu to appear (TODO: insert wait time here), click new letter in swap menu at `x3,y3` and wait for swap move to complete (TODO: insert wait time here).
- `MOVE x1 y1 x2 y2 [xN yN...]` - making actual move; you'll need to move mouse to `x1,y1`, wait for around 50 ms, hold primary mouse button, move to `x2,y2` while still holding it, wait for around 50 ms, repeat until coordinates end, release primary mouse button.

This is example implementation made as bash script (using `xdotool` for mouse) that reads commands from stdin:

```bash
# TODO: insert script here
```

## Complete script

At the end you just need to combine everything into single command/script:

```bash
spectacle -fbno /tmp/spellshot.png && ./target/release/spellcast-solver automatic -i /tmp/spellshot.png -x 22 -y 347 | ./mouse_commands.sh
```

## Running

Now you'll need a way to quickly run it while in game (make sure everything is stable: it's your turn, no animations happen on board), either with keybind or by having terminal nearby.

## What it does

This tool will use screenshot to parse complete game state:
- Board (letters, 2x/3x, DL/TL, gems)
- Your current gem count

It will solve the board with maximum available depth (e.g. 2 swaps if you have 7 gems) and return mouse instructions for best move (as forcing user to pick a move would defeat the whole purpose of it being automatic).
Your wrapper script will use mouse instructions to actually make a move, and the cycle will repeat when it's your turn again.

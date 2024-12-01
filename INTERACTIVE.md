# Interactive solver guide

## Welcome screen

When starting the interactive folder, you'll see something like this:

```
The superior Discord Spellcast solver - now in Rust!
(c) 2024 Woidly (MIT license)
https://github.com/Woidly/spellcast-solver

Welcome to interactive mode!
You can learn more about it from INTERACTIVE.md.
Terminal size of at least 14x52 is required.
Basic 16-colour support is recommended.

[S]tart | [Ctrl+C/Ctrl+Z] Exit
```

Not much to do here, but it teaches you some basics:

1. If word has its letter inside of brackets, it is keybind (e.g. S = Start)
2. One action can have multiple keybinds separated by `/`
3. You can press **`Ctrl+C`**/**`Ctrl+Z`** at any point in program (except when it's solving board) to quit it

Got it? Press **`S`** and let's continue!

## Letter editor

```
# A B C D E  State::LetterEditor
1 ? ? ? ? ?  pos: 0 (A1)
2 ? ? ? ? ?
3 ? ? ? ? ?
4 ? ? ? ? ?
5 ? ? ? ? ?

[Arrow keys] Move cursor | [A-Z] Change letter
[#] Frozen tile
[Ctrl+C/Ctrl+Z] Exit
```

Now THIS is a real solver menu!
Rest of menus will have this similar layout:

- Board in top-left corner
- Current state on first row
- Additional meta (in this case, position) on next 5 rows
- Available keybinds at the bottom

On first start, the cursor (in actual terminal tile will be highlighted red) will be at first tile (A1).

Pressing **`A`**-**`Z`** will change letter on the current tile and move cursor to the next ? (empty) tile.
This will allow you to type the entire board (or fill empty spaces after move) by just pressing letter keys.
But if there are no ? tiles, cursor will stay in place, making editing a complete board more predictable.

Pressing **`#`** will do the same, but instead of changing tile's letter, it will set tile.frozen to true.
As frozen tiles are completely ignored by solver, there is no need to fill out what letter is in this tile.
Pressing **`A`**-**`Z`** on frozen tile will revert tile.frozen to false.

In case of typo, you can use arrow keys to get back to edited tile and press letter key again (it will overwrite letter on tile).

When there are no more ? tiles left, new option will appear - `[Enter] Done`.
Pressing **`Enter`** will move you to [`Normal` state](#normal).

## Normal

```
# A B C D E  State::Normal
1 h e l l o  3 gems (1 swaps)
2 w o r l d  (+0 score per gem)
3 i a m w o
4 i d l y o
5 k b y e e

Edit board: [L]etters | [G]ems | [N]ew game
Letter multiplier: [0] Remove | [+] DL | [*] TL
Word multiplier: [1] Remove | [$/2] 2x | [^/3] 3x
Edit meta: Gem [C]ount | Gem score [B]onus
[S]olve | [Ctrl+C/Ctrl+Z] Exit
```

- Pressing **`L`** will return you back to [letter editor](#letter-editor), described above, where you can edit current board (e.g. correct a typo)
- Pressing **`G`** will open [gem editor](#gem-editor)
- Pressing **`N`** will completely wipe the board and return everything to defaults (like restarting the program), except for the gem bonus
- Pressing **`0`** removes letter multiplier, no matter where it is located
- Pressing **`+`** opens [tile picker](#tile-picker) and will set selected tile's letter multiplier to 2 (DL)
- Pressing **`*`** does the same, but letter multiplier is set to 3 (TL)
- Pressing **`1`** removes word multiplier, no matter where it is located
- Pressing **`$`** or **`2`** opens [tile picker](#tile-picker) and will set selected tile's word multiplier to 2x
- Pressing **`^`** or **`3`** does the same, but word multiplier is set to 3x
- Pressing **`C`** opens [number picker](#number-picker) and changes current gem count to specified number (swap count is based on it)
- Pressing **`B`** opens [number picker](#number-picker) and changes additional score added to tiles with gems to specified number
- Pressing **`S`** starts solving the board and switches to [`Solved` state](#solved) when it's done

Most keybinds will work regardless of the Shift state, e.g. letters are case-insensitive and you can press **`4`** instead of **`$`** (**`Shift+4`**) or **`!`** (**`Shift+1`**) instead of **`1`**. Note that these mappings are hardcoded and based on the en-US QWERTY layout, so other layouts may require pressing the exact character.

## Gem editor

```
# A B C D E  State::GemEditor
1 h e l l o  pos: 6 (B2)
2 w o!r l!d  gem: true
3 i a m!w o!
4 i d!l y!o
5 k b y e e

[Arrow keys] Move cursor | [G/!] Toggle gem
[Enter] Done
[Ctrl+C/Ctrl+Z] Exit
```

It is very similar to [letter editor](#letter-editor), except you can always exit it with **`Enter`** and instead of changing letters you flip gem state on selected tile with **`G`**/**`!`**.

If a tile has a gem, it'll have green `!` after its letter.
There's also boolean indicator of current tile's gem under current position.

Specifying gem tiles is completely optional, but it will allow you to prioritise tiles with gems (press **`B`** in the [normal menu](#normal)) and automatically handle the swap count. Otherwise you'll have to manually specify new gem count after each move to use swaps in next move (press **`C`** in [normal menu](#normal)).

## Tile picker

```
# A B C D E  State::PickTile(...)
1 h e l l o
2 w o r l d
3 i a m w o
4 i d l y o
5 k b y e e

[A-E] Choose column | [1-5] Choose row
[Esc/Z] Back | [Ctrl+C/Ctrl+Z] Exit
```

It is pretty simple, you just press column and row keys of tile you want to pick (e.g. **`A`** and **`1`** for first tile).
After both column and row have been chosen, new option appears - `[Enter] Done`.
Pressing **`Enter`** does whatever action you've previously selected with chosen tile.
But if you changed your mind (or just did a typo), you can press **`Esc`** or **`Z`** to return to the [normal menu](#normal) as if nothing had happened.

> [!TIP]
> You can see which exact action will be done in parenthesis after state (e.g. `State::PickTile(TL)` or `State::PickTile(TwoX)`)

## Number picker

```
# A B C D E  State::PickNumber(...)
1 h e l l o
2 w o r l d
3 i a m w o
4 i d l y o
5 k b y e e

[0-9] Choose 0-9 | [-] Choose 10
[Esc/Z] Back | [Ctrl+C/Ctrl+Z] Exit
```

Board isn't used in number picker, I just left it here to keep consistency between menus.
You just need to press the corresponding number key (**`0`** to **`9`** for 0-9 or **`-`** for 10) and it will do whatever action you've previously selected with this number.

Similarly to tile picker, you can go back with **`Esc`**/**`Z`** and see action in state indicator.

## Solved

```
# A B C D E  State::Solved
1 h e l l o  33.80ms elapsed
2 w o r l d
3 i a m w o
4 i d l y o
5 k b y e e

[1] hollowware (+36) | [2] fellowly (+35)
[3] hollowly (+34) | [4] mellowly (+34)
[5] yellowly (+34) | [6] wormfly (+34)
[7] wormwoods (+34) | [8] maxwell (+34)
[9] eelwrack (+34) | [0] wormwood (+32)
[U]nsolve | [Ctrl+C/Ctrl+Z] Exit
```

After solving the board you will be sent to this menu.
Below state indicator you can see amount of time spent on solving.

- By pressing **`U`** you can go back to [`Normal` state](#normal) (in case you realised you made a mistake)
- By pressing **`1`**-**`9`** and **`0`** you can select one of words listed below. There is score in parenthesis after each word. If some letter in word was changed via swap, it will be highlighted in red. Selecting a word will switch you to [`Move` state](#move)

## Move

```
# A B C D E  State::Move(0)
1 h0e9l2* *  hollowware
2 * o1r8l3*  +36 points
3 * a7w6w5o4 1 swaps used
4 * * * * *  0 gems collected
5 * * * * *  0 gems after move

^ Now make the move following this instruction
Number on the right of tile is step number
Two-digit numbers use hexadecimal letters (e.g. 14=e)
[A]ccept
[Esc/Z] Back | [Ctrl+C/Ctrl+Z] Exit
```

This screen contains detailed information about a word - sequence of letters to make it, how many swaps were used, how many gems were collected, and how many gems you'll have after making this move.

On example board you can see that unrelated tiles have turned into `*`, and every letter now has a number (or a letter) after it. This is exact order of moves you'll need to make (note that counter starts from 0). If move number is larger than 9, it will become a hexadecimal letter (10=a, 11=b, 12=c, etc.).

If you made that move in Spellcast, press **`A`**. It will update your gem count accordingly, remove tiles used in move and return you to [letter editor](#letter-editor).
If you changed your mind, press **`Esc`** or **`Z`** to return to [`Solved` state](#solved) (don't worry, it won't require re-solving the board).

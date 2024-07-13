# Vim Motions Cheat Sheet

## Basic Movement
- `h`: Move left
- `j`: Move down
- `k`: Move up
- `l`: Move right
- `w`: Move to the start of the next word
- `b`: Move to the start of the previous word
- `e`: Move to the end of the current/next word
- `0`: Move to the start of the line
- `$`: Move to the end of the line
- `^`: Move to the first non-blank character of the line

## Jumping
- `gg`: Go to the first line of the document
- `G`: Go to the last line of the document
- `{number}G`: Go to a specific line number
- `Ctrl+u`: Page up (move cursor up half a screen)
- `Ctrl+d`: Page down (move cursor down half a screen)
- `%`: Jump to matching parenthesis/bracket

## Searching
- `/pattern`: Search forward for pattern
- `?pattern`: Search backward for pattern
- `n`: Repeat search in same direction
- `N`: Repeat search in opposite direction
- `*`: Search for word under cursor (forward)
- `#`: Search for word under cursor (backward)

## Editing
- `i`: Insert before cursor
- `a`: Insert after cursor
- `I`: Insert at beginning of line
- `A`: Insert at end of line
- `o`: Open new line below current line
- `O`: Open new line above current line

## Deleting
- `x`: Delete character under cursor
- `dd`: Delete current line
- `dw`: Delete from cursor to start of next word
- `d$` or `D`: Delete from cursor to end of line
- `d0`: Delete from cursor to start of line

## Changing (Delete and Enter Insert Mode)
- `cw`: Change from cursor to start of next word
- `c$` or `C`: Change from cursor to end of line
- `cc`: Change entire line

## Yanking (Copying) and Putting (Pasting)
- `yy`: Yank current line
- `yw`: Yank from cursor to start of next word
- `y$`: Yank from cursor to end of line
- `p`: Put (paste) after cursor
- `P`: Put (paste) before cursor

## Visual Mode
- `v`: Enter character-wise visual mode
- `V`: Enter line-wise visual mode
- `Ctrl+v`: Enter block-wise visual mode

## Marks
- `ma`: Set mark 'a' at current cursor position
- `'a`: Jump to line of mark 'a'
- ``a`: Jump to position of mark 'a'

## Macros
- `qa`: Start recording macro 'a'
- `q`: Stop recording macro
- `@a`: Execute macro 'a'
- `@@`: Repeat last executed macro
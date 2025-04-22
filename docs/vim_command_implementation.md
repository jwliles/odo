# Vim Command Implementation Tracking

This document tracks which Vim commands we're implementing, categorized by:
1. Commands to preserve exactly as in Vim
2. Commands to enhance with Org-specific functionality 
3. New Org-specific commands with no Vim equivalent

## Standard Vim Commands to Preserve (Phase 0)

### Movement
- `h`, `j`, `k`, `l` - Basic character movement
- `w`, `b`, `e` - Word-based movement
- `0`, `^`, `$` - Line navigation
- `gg`, `G` - Document start/end
- `{`, `}` - Paragraph movement
- `Ctrl-d`, `Ctrl-u` - Half-page movement
- `Ctrl-f`, `Ctrl-b` - Full-page movement
- `%` - Jump to matching bracket
- `fx`, `Fx`, `tx`, `Tx` - Find character on line
- `;`, `,` - Repeat last find

### Editing
- `i`, `I`, `a`, `A`, `o`, `O` - Insert mode variations
- `x`, `X` - Delete character
- `dd`, `D` - Delete line/to end of line
- `cc`, `C` - Change line/to end of line
- `yy`, `Y` - Yank line
- `p`, `P` - Put text
- `r` - Replace character
- `~` - Toggle case
- `J` - Join lines
- `u`, `Ctrl-r` - Undo/redo
- `.` - Repeat last change

### Search
- `/`, `?` - Search forward/backward
- `n`, `N` - Repeat search
- `*`, `#` - Search for word under cursor

### Visual Mode
- `v`, `V`, `Ctrl-v` - Character, line, block visual mode
- Visual mode operations (y, d, c, etc.)

### Command Mode
- `:w`, `:q`, `:wq` - Save, quit, save and quit

## Commands to Enhance with Org-specific Functionality (Phase 1)

### Movement
- `]]`, `[[` - Navigate between sections at same level
- `gj`, `gk` - Navigate by visual lines in wrapped text
- `zz`, `zt`, `zb` - Recenter screen (with Org context awareness)

### Editing
- `>>`, `<<` - Indent/outdent with Org heading awareness
- `>`, `<` in visual mode - Org-aware indent/outdent

### Folding
- `za`, `zA` - Toggle fold at cursor
- `zc`, `zC` - Close fold at cursor
- `zo`, `zO` - Open fold at cursor
- `zM`, `zR` - Close/open all folds

## New Org-specific Commands (Phase 2)

### Structural Navigation (g prefix)
- `gh`, `gl` - Previous/next heading at same level
- `gj`, `gk` - Next/previous heading (any level)
- `gt` - Jump to parent heading
- `gc` - Jump to next code block
- `gC` - Jump to previous code block
- `gl` - Jump to next list item
- `gL` - Jump to previous list item
- `gd` - Jump to deadline/scheduled date
- `gT` - Jump to next TODO item
- `gp` - Jump to next property drawer

### Structure Editing
- `<leader>h` - Create new heading
- `<leader>s` - Create new subheading
- `<leader>t` - Cycle TODO state
- `<leader>p` - Add property
- `<leader>d` - Add deadline
- `<leader>c` - Create code block
- `<leader>>`, `<leader><` - Promote/demote heading
- `<leader>m` - Move subtree up
- `<leader>M` - Move subtree down

### Agenda/Dates
- `<leader>da` - Archive subtree
- `<leader>dc` - Add creation date
- `<leader>dd` - Add deadline
- `<leader>ds` - Add scheduled date

### Code Blocks
- `<leader>ce` - Execute code block
- `<leader>cr` - Edit code block results
- `<leader>cs` - Set code block parameters

## Implementation Status

| Command | Category | Status | Notes |
|---------|----------|--------|-------|
| `h`,`j`,`k`,`l` | Standard | Pending | Basic movement |
| `w`,`b`,`e` | Standard | Pending | Word movement |
| `0`,`^`,`$` | Standard | Pending | Line navigation |
| ... | ... | Pending | ... |
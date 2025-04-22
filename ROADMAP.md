# NeoOrg Roadmap

## ✅ Implemented Features
- Basic text editing (open, edit, save files)
- Modal editing (Command/Insert modes with Vi-like keybindings)
- Basic navigation (arrow keys, Home/End, PgUp/PgDn)
- Enhanced search functionality (Ctrl-f):
  - Incremental search as you type
  - Navigation between matches (arrows, j/k, n/p)
  - Search wraparound when reaching document boundaries
  - Highlighted search results
- Open files from within the editor (Ctrl-o)
- Status bar with file info and mode indicator
- Org-mode syntax highlighting:
  - Headlines with levels
  - TODO/DONE states
  - Basic text formatting (bold, italic, underline)
  - Simple list detection
  - Tags
  - Links

## 🔄 Currently Working On
- Modular architecture refinement (core, editor, UI components)
- Terminal UI improvements
- GUI mode implementation
- Org-mode-aware Vim motions planning and initial implementation

## ⏭️ Next Up (Short Term)
- Complete Org-mode syntax highlighting
- Folding/unfolding sections
- Properties drawers support
- Task states customization

### Org-optimized Motions
- Efficient keyboard navigation tailored for Org-mode:
  - Jump between headings with `gh` (previous) and `gj` (next)
  - Navigate to parent heading with `gp`, child heading with `gc`
  - Move between list items with `gl` (previous) and `gn` (next)
  - Jump to TODO items with `gt` (next) and `gT` (previous)
  - Navigate to tags quickly with `g#`
  - Jump to code blocks with `gb` (beginning) and `ge` (end)
  - Navigate between matching blocks with `gm` (match)
  - Toggle folding with `z` (expand/collapse current heading)

- Standard motion compatibility:
  - Preserve essential Vim motions (`/`, `*`, `gg`, `G`, etc.)
  - Maintain familiar navigation for users with Vim experience
  - Keep common text operations consistent with Vim
  - See detailed implementation tracking in `docs/vim_command_implementation.md`

### Structural Text Operations
- Efficient text operations for structured editing:
  - `dh` - delete heading (including content)
  - `dl` - delete list item
  - `dc` - delete code block
  - `yh` - yank (copy) heading
  - `yl` - yank list item
  - `yc` - yank code block
  - `ch` - change heading text (preserving level)

## 🔍 Medium Term
- GUI interface implementation
- Tables with calculations
- Code blocks with language-specific highlighting
- Tree-sitter integration for improved parsing
- Agenda views
- Task management features
- Priority markers
- Deadlines and scheduled dates

### Org Structure Manipulation
- Powerful shortcuts for restructuring documents:
  - `+h` and `-h` to promote/demote headings
  - `+l` and `-l` to promote/demote list items
  - `tt` to cycle through TODO states
  - `#` to add/edit tags
  - Automatic list continuation
  - Structure-aware selection with `v`
  - `m` to move headings or sections
  - `s` to sort headings by priority/status

## 🔮 Long Term
- Export to HTML/Markdown/PDF
- Calendar integration
- Time tracking
- Plugin system
- Mobile companion app
- Synchronization with other Org-mode tools
- External editor integration (Emacs, Neovim)

This roadmap is a living document and will be updated as development progresses.
# Odo Roadmap

*A versatile text editor with first-class Org support, designed for code, prose, and any content.*

## ✅ Implemented Features
- **Core Text Editing**
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

- **Org-mode Support**
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
- Content-aware editing features

## ⏭️ Next Up (Short Term)
- **Multi-language Support**
  - Basic syntax highlighting for common languages (Rust, Python, JavaScript, etc.)
  - Language detection based on file extension
  - Configurable language associations

- **Enhanced Org Support**
  - Complete Org-mode syntax highlighting
  - Folding/unfolding sections
  - Properties drawers support
  - Task states customization
  - **Org-babel (Literate Programming)**
    - Code block execution with result capture
    - Basic tangling (extract code blocks to files)
    - Support for common languages (Python, JavaScript, Rust, Shell)
    - Inline result display

- **General Editing Features**
  - Line numbers toggle
  - Word wrap support
  - Multiple cursors (basic implementation)
  - Bracket matching and auto-completion

- **Advanced Text Manipulation (Emacs-inspired)**
  - Kill ring - circular buffer for cut/copied text with cycling
  - Undo regions - visual undo with selective region restoration
  - Rectangular selection and editing
  - Transpose operations (characters, words, lines)
  - Case conversion commands (capitalize, upcase, downcase)
  - Advanced search and replace with regex support

### Content-Aware Motions
- **Org-mode Navigation**
  - Jump between headings with `gh` (previous) and `gj` (next)
  - Navigate to parent heading with `gp`, child heading with `gc`
  - Move between list items with `gl` (previous) and `gn` (next)
  - Jump to TODO items with `gt` (next) and `gT` (previous)
  - Navigate to tags quickly with `g#`
  - Toggle folding with `z` (expand/collapse current heading)

- **Code Navigation**
  - Jump between functions/methods with `gf` (next) and `gF` (previous)
  - Navigate to matching brackets with `gm`
  - Jump to code blocks with `gb` (beginning) and `ge` (end)
  - Navigate between classes/structs with `gc` (next) and `gC` (previous)

- **Standard Vim Compatibility**
  - Preserve essential Vim motions (`/`, `*`, `gg`, `G`, etc.)
  - Maintain familiar navigation for users with Vim experience
  - Keep common text operations consistent with Vim
  - See detailed implementation tracking in `docs/vim_command_implementation.md`

### Content-Aware Text Operations
- **Org-mode Operations**
  - `dh` - delete heading (including content)
  - `dl` - delete list item
  - `yh` - yank (copy) heading
  - `yl` - yank list item
  - `ch` - change heading text (preserving level)
  - **Org-babel Operations**
    - `<leader>be` - execute code block at cursor
    - `<leader>bt` - tangle current file (extract all code blocks)
    - `<leader>bs` - execute code block in session
    - `<leader>br` - remove execution results
    - `<leader>ba` - execute all code blocks in document

- **Code Operations**
  - `df` - delete function/method
  - `dc` - delete code block
  - `yf` - yank function/method
  - `yc` - yank code block
  - `cf` - change function signature

- **Advanced Text Operations**
  - **Kill Ring Integration**: `M-y` to cycle through kill ring after paste
  - **Undo Regions**: Visual selection of undo boundaries with `C-u C-/`
  - **Rectangular Editing**: `C-x r` prefix for rectangle operations
  - **Transpose Commands**: `M-t` (words), `C-t` (chars), `C-x C-t` (lines)
  - **Case Operations**: `M-u` (upcase), `M-l` (downcase), `M-c` (capitalize)

## 🔍 Medium Term
- **Advanced Language Support**
  - Tree-sitter integration for improved parsing
  - Language servers integration (LSP support)
  - Code completion and error diagnostics
  - Refactoring tools
  - Symbol navigation (go to definition, find references)

- **Enhanced GUI**
  - Full-featured GUI interface
  - Split panes and tabs
  - File explorer
  - Project management
  - Theme customization

- **Best-of-Breed Editor Features**
  - **From VSCode**: Command palette, integrated terminal, extensions marketplace
  - **From Vim**: Advanced macros, registers, visual block mode improvements
  - **From Sublime**: Goto anything, minimap, column selection
  - **From IntelliJ**: Smart refactoring, code generation, live templates
  - **From Kakoune**: Multiple selections, client-server architecture
  - **From Helix**: Tree-sitter selections, space-based commands

- **Org-mode Advanced Features**
  - Tables with calculations
  - Agenda views
  - Task management features
  - Priority markers
  - Deadlines and scheduled dates
  - **Advanced Org-babel**
    - Session-based execution (persistent state across blocks)
    - Complex tangling with noweb references
    - Result caching and incremental evaluation
    - Support for 20+ programming languages
    - Graphics and plot generation
    - Database integration (SQL execution)
    - Notebook-style interactive development

### Content Structure Manipulation
- **Org-mode Structure**
  - `+h` and `-h` to promote/demote headings
  - `+l` and `-l` to promote/demote list items
  - `tt` to cycle through TODO states
  - `#` to add/edit tags
  - Automatic list continuation
  - Structure-aware selection with `v`
  - `m` to move headings or sections
  - `s` to sort headings by priority/status

- **Code Structure**
  - Automatic indentation
  - Code folding
  - Function/class extraction
  - Import organization
  - Format on save

## 🔮 Long Term
- **Extensibility**
  - Plugin system with hot-reloading
  - Custom language support
  - User-defined commands and shortcuts
  - Scriptable automation (Lua/JavaScript)

- **Power User Features**
  - **From Emacs**: Lisp-based configuration, powerful macros, buffer management
  - **From Vim**: Ex commands, substitute patterns, global commands
  - **From Acme**: Mouse chording, everything-is-a-file philosophy
  - **From Xi**: Operational transforms, CRDT-based collaboration
  - **From Zed**: GPU acceleration, collaborative editing, fast search

- **Advanced Features**
  - Export to HTML/Markdown/PDF
  - Version control integration (Git)
  - Collaborative editing
  - Cloud synchronization
  - Cross-platform mobile support

- **Org-mode Integration**
  - Calendar integration
  - Time tracking
  - Synchronization with other Org-mode tools
  - External editor integration (Emacs, Neovim)

- **Developer Tools**
  - Debugging integration
  - Build system integration
  - Testing framework support
  - Documentation generation

---

*This roadmap reflects Odo's vision as a versatile text editor that excels at both general-purpose editing and specialized Org-mode workflows. The roadmap is a living document and will be updated as development progresses.*
# NeoOrg

A standalone Org-mode editor written in Rust, inspired by both Vim and Emacs. NeoOrg provides Org-mode functionality without requiring Emacs while maintaining a modal editing style similar to Vim.

## Features

- Command and Insert modes with Vi-like keybindings
- Org-mode syntax highlighting and functionality:
  - Headlines with different levels (*, **, ***, etc.)
  - TODO state tracking (TODO, DONE)
  - Tags support (e.g., :work:, :personal:)
  - Code blocks with syntax highlighting
  - Lists (ordered, unordered, checklists)
- File operations (open, save)
- Advanced search functionality
- Agenda views and task management
- Export capabilities (HTML, Markdown)

## Usage

```
cargo run [filename]
```

### Key Commands

**Command Mode (default):**
- `i` - Enter insert mode
- `a` - Enter insert mode after cursor
- `A` - Enter insert mode at end of line
- `I` - Enter insert mode at beginning of line
- `o` - Open new line below and enter insert mode
- `O` - Open new line above and enter insert mode
- `x` - Delete character under cursor
- Arrow keys - Move cursor
- `Home`/`End` - Move to start/end of line
- `PageUp`/`PageDown` - Move up/down by one screen
- `Ctrl-f` - Search text
- `Ctrl-s` - Save file
- `Ctrl-o` - Open file
- `Ctrl-q` - Quit (press multiple times if file has unsaved changes)

**Insert Mode:**
- `Esc` - Return to command mode
- Type to insert text
- `Backspace` - Delete character before cursor
- `Delete` - Delete character under cursor
- Arrow keys - Move cursor
- `Ctrl-s` - Save file
- `Ctrl-o` - Open file
- `Ctrl-q` - Quit (press multiple times if file has unsaved changes)

## Development

This project follows the GitFlow workflow:

- `main` - Production code
- `develop` - Development branch where features are integrated
- `feature/xxx` - Feature branches for new functionality
- `hotfix/xxx` - Hotfix branches for urgent production fixes
- `release/x.x.x` - Release branches for preparing new versions

### Build Instructions

```
cargo build --release
```

## Org-mode Functionality

NeoOrg implements the core functionality of Emacs Org-mode in a standalone application with a modal editing interface inspired by Vim. Key Org-mode features include:

### Document Structure
- Headlines with different levels for organizing content
- Folding/unfolding of sections
- Properties drawers for metadata

### Task Management
- TODO keywords for tracking task states
- Priority markers
- Deadlines and scheduled dates
- Tags for categorization

### Content Features
- Tables with spreadsheet-like calculations
- Code blocks with syntax highlighting for various languages
- Links to files, websites, or within the document
- Lists (ordered, unordered, and checklists)

### Advanced Features
- Agenda views to see scheduled tasks and deadlines
- Export functionality to convert Org files to other formats
- Calendar integration
- Time tracking

## Why NeoOrg?

NeoOrg aims to bring the power of Org-mode to users who:
- Prefer Vim-style modal editing
- Want a lightweight, fast application focused solely on Org-mode
- Need cross-platform compatibility with modern systems
- Desire a customizable experience without the complexity of Emacs configuration

## License

MIT
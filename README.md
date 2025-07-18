# Odo

A versatile text editor with first-class Org support, designed for code, prose, and any content. Odo combines the power of modal editing with excellent support for Org-mode, making it perfect for both programming and writing tasks.

## Features

### Currently Implemented
- Command and Insert modes with Vi-like keybindings
- Basic file operations (open, save)
- Simple search functionality
- GUI mode (experimental, run with `--gui` flag)

### Partially Implemented
- Basic Org-mode syntax highlighting:
  - Headlines detection for different levels (*, **, ***, etc.)
  - TODO/DONE keyword detection
  - Tag highlighting (both in headlines and standalone tag lines)
  - Basic list item detection
  - Text styling indicators (* for bold, / for italic, _ for underline)
  - Basic link detection

### Planned for Future Releases
- Full Org-mode functionality:
  - Enhanced syntax highlighting
  - Code blocks with syntax highlighting
  - Checklists
  - Folding/unfolding of sections
- Multi-language syntax highlighting for code files
- Agenda views and task management
- Export capabilities (HTML, Markdown)
- Tags filtering and management
- Tables with spreadsheet-like calculations
- Advanced text editing features for prose writing

## Usage

### Terminal Mode (Default)
```
cargo run [filename]
```

### GUI Mode (Experimental)
```
cargo run -- --gui [filename]
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

### Current Development Focus

The current development focus is on:
1. Completing the modular architecture
2. Improving the terminal UI experience
3. Developing the experimental GUI mode
4. Implementing Org-mode-aware Vim motions and text objects

See the [ROADMAP.md](ROADMAP.md) file for more details on planned features.

### Build Instructions

```
cargo build --release
```

## Org-mode Functionality

Odo aims to implement the core functionality of Emacs Org-mode in a standalone application with a modal editing interface inspired by Vim.

### Current Progress
- **Document Structure**
  - Basic detection of headlines with different levels
  - Simple syntax highlighting for key Org elements

- **Formatting**
  - Basic highlighting for text formatting indicators (bold, italic, underline)
  - Rudimentary detection of links

- **Task Management**
  - Simple highlighting for TODO/DONE keywords
  - Basic detection of lists and tags (visual highlighting only)

### Planned for Future Releases
- **Document Structure**
  - Folding/unfolding of sections
  - Properties drawers for metadata
  - Better handling of hierarchical structure

- **Task Management**
  - Priority markers
  - Deadlines and scheduled dates
  - Enhanced TODO state workflows
  - Functional tags system

- **Content Features**
  - Tables with spreadsheet-like calculations
  - Code blocks with syntax highlighting for various languages
  - Functional links to files, websites, or within the document
  - Interactive checklists

- **Advanced Features**
  - Agenda views to see scheduled tasks and deadlines
  - Export functionality to convert Org files to other formats
  - Calendar integration
  - Time tracking

## Why Odo?

Odo is a versatile text editor that excels at:
- Efficient modal editing inspired by Vim's philosophy
- First-class Org-mode support for organization and note-taking
- Lightweight, fast performance for any type of content
- Cross-platform compatibility with modern systems
- Intuitive experience without complex configuration
- Flexibility with both terminal and graphical interfaces
- Seamless editing of code, prose, documentation, and structured content

## Requirements

- Rust (Minimum supported version: 1.70.0)
- GNU/Linux or other free operating system
- Standard system libraries

**Note**: Odo is developed exclusively for free operating systems. It is not officially tested or supported on proprietary platforms.

## License

MIT
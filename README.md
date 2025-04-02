# Orgonaut

A modal text editor written in Rust, inspired by Vim.

## Features

- Command and Insert modes
- Syntax highlighting
- File operations (open, save)
- Search functionality
- Vi-like keybindings

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
- `Ctrl-q` - Quit (press multiple times if file has unsaved changes)

**Insert Mode:**
- `Esc` - Return to command mode
- Type to insert text
- `Backspace` - Delete character before cursor
- `Delete` - Delete character under cursor
- Arrow keys - Move cursor
- `Ctrl-s` - Save file
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

## License

MIT
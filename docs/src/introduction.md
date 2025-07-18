# Introduction to Odo

Odo is a standalone Org-mode editor written in Rust, inspired by both Vim and Emacs. It provides Org-mode functionality without requiring Emacs while maintaining a modal editing style similar to Vim.

## Why Odo?

Odo aims to bridge the gap between Emacs Org-mode's powerful organizational capabilities and Vim's efficient editing paradigm. It's designed for users who:

- Appreciate efficient, modal editing inspired by Vim's philosophy
- Want a lightweight, fast application focused solely on Org-mode
- Need cross-platform compatibility with modern systems
- Desire an intuitive experience without the complexity of Emacs configuration
- Want flexibility with both terminal and graphical interfaces
- Would benefit from keyboard shortcuts specifically optimized for Org-mode structure

## Key Features

- **Modal Editing**: Command and Insert modes with Vi-like keybindings
- **Org-mode Syntax**: Support for headlines, TODO items, tags, lists, and text styling
- **File Operations**: Open, edit, and save Org files
- **Multiple Interfaces**: Both terminal and GUI modes
- **Search Functionality**: Find text within documents

## Org-mode Functionality

Odo implements core Org-mode features including:

- **Document Structure**: Headlines with different levels and properties
- **Text Formatting**: Bold, italic, underline and other text styling
- **Task Management**: TODO/DONE states, priorities, and tags
- **Lists**: Ordered, unordered, and description lists
- **Links**: Internal and external reference linking

## Interfaces

Odo offers two main interfaces:

- **Terminal Mode**: A lightweight TUI for efficient terminal-based editing
- **GUI Mode**: A graphical interface with additional visual features

## System Requirements

- Rust (Minimum supported version: 1.70.0)
- GNU/Linux or other free operating system
- Standard system libraries

## License

Odo is released under the MIT License. See the [LICENSE](https://github.com/jwliles/odo/blob/main/LICENSE) file for details.
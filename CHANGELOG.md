# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Open files from within the editor with Ctrl+o in both Command and Insert modes
- Confirmation prompt when opening a new file with unsaved changes

### Improved
- Enhanced search functionality (Ctrl-f):
  - Incremental search as you type
  - Navigation between matches with arrow keys, j/k, or n/p
  - Search wraparound when reaching document boundaries
  - Added feedback messages for search status
  - Improved search result highlighting

## [0.2.0] - 2025-04-02

### Added
- Command mode implementation (start in command mode by default)
- Status bar mode indicator showing current mode (INSERT/COMMAND)
- Vim-like key bindings: i, a, A, I, o, O for entering insert mode
- ESC key to return to command mode from insert mode
- Updated help message for command mode navigation

## [0.1.0] - 2025-04-02

### Added
- Basic text editor functionality
- Syntax highlighting
- File operations (open, save)
- Search functionality
- Basic navigation with arrow keys, Home, End, PgUp, PgDn
- Status bar with file information
- Message bar for user prompts
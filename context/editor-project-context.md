# Text Editor Project Context

## Project Goals
- **High Performance**: All operations under 16ms
- **Beauty**: Modern UI with excellent text rendering and Unicode support
- **Reliability**: No crashes, hangs, or lost work
- **Developer Friendliness**: Easily customizable through plugins or core modifications

## Design Decisions
- **Front-end/Back-end Separation**: UI separate from text processing core
- **Native UI**: Using platform-specific UI frameworks for best look and feel
- **Rust**: Primary language for performance and reliability
- **Persistent Rope Data Structure**: For efficient text manipulation
- **Asynchronous Operations**: Never block the UI thread
- **Plugins over Scripting**: Communication through pipes and JSON
- **JSON**: For all inter-component communication

## Language Considerations
Languages evaluated:
- **Rust**: Strong memory safety, performance, concurrency
- **OCaml**: Strong type system, functional paradigm, but GC concerns
- **Zig**: Simple, performant, but younger ecosystem
- **D**: Good C++ interop, unit testing, but less active community
- **Odin**: Clean syntax, performance, but very young ecosystem

Current choice: **Rust**

## Multi-Frontend Support
- Terminal interface (priority)
- GUI interface
- Possibly web interface later

## Proposed Project Structure
```
src/
├── main.rs                 # Application entry point
├── core/                   # Backend module
│   ├── buffer/             # Text buffer with rope structure
│   ├── document/           # Document management
│   ├── operations/         # Text editing operations
│   ├── io/                 # File operations
│   └── search/             # Search and replace
├── protocol/               # Communication layer
│   ├── messages.rs         # Message definitions
│   ├── serialization.rs    # JSON handling
│   └── server.rs           # Protocol coordination
├── frontends/              
│   ├── common/             # Shared frontend code
│   ├── terminal/           # Terminal UI frontend (priority)
│   └── gui/                # Graphical frontend
├── plugins/                # Plugin system
├── config/                 # Configuration
└── utils/                  # Shared utilities
```

## Recommended Next Steps
1. Implement the core with basic rope data structure
2. Create a minimal terminal frontend first
3. Establish the protocol layer between them
4. Implement basic file I/O
5. Add initial undo/redo system
6. Expand with more features once the foundation is solid

## Technical Considerations
- Start simple before adding Tree-sitter or complex undo systems
- Focus on a "walking skeleton" with basic editing functionality
- Use libraries like ratatui/crossterm for the terminal UI
- Use serde_json for protocol serialization

## Milestones
1. **Core Text Editing**: Basic buffer, operations, minimal terminal UI
2. **File Operations**: Open/save, async I/O, basic config
3. **Editor Experience**: Undo/redo, search/replace, UI polish

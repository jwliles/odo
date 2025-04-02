orgonaut/  
├── Cargo.toml  
├── .github/                    # CI/CD setup  
├── docs/                       # Documentation   
├── assets/                     # Icons, themes, etc.  
├── src/  
│   ├── main.rs                 # Entry point that selects UI based on args  
│   ├── core/                   # Core functionality shared by all UIs  
│   │   ├── mod.rs  
│   │   ├── config.rs           # Configuration handling  
│   │   ├── workspace.rs        # Multi-file workspace management  
│   │   ├── vault.rs            # File storage and management (like Obsidian vault)  
│   │   ├── plugin.rs           # Plugin system architecture  
│   │   ├── commands.rs         # Command registry for editor operations  
│   │   └── events.rs           # Event system for inter-component communication  
│   │  
│   ├── orgmode/                # Org-mode specific functionality  
│   │   ├── mod.rs  
│   │   ├── parser/             # Parse Org-mode files  
│   │   │   ├── mod.rs  
│   │   │   ├── lexer.rs        # Tokenize Org syntax  
│   │   │   ├── grammar.rs      # Define formal Org grammar  
│   │   │   └── ast.rs          # Abstract Syntax Tree for Org documents  
│   │   │  
│   │   ├── model/              # Document object model  
│   │   │   ├── mod.rs  
│   │   │   ├── document.rs     # Document structure  
│   │   │   ├── headline.rs     # Headlines with properties  
│   │   │   ├── list.rs         # List items  
│   │   │   ├── table.rs        # Table management  
│   │   │   ├── markup.rs       # Text formatting  
│   │   │   ├── link.rs         # Link handling  
│   │   │   └── block.rs        # Special blocks (src, example, etc.)  
│   │   │  
│   │   ├── operations/         # Operations on Org documents  
│   │   │   ├── mod.rs  
│   │   │   ├── edit.rs         # Basic editing  
│   │   │   ├── move.rs         # Moving elements  
│   │   │   ├── structure.rs    # Structural editing  
│   │   │   ├── search.rs       # Search functionality  
│   │   │   ├── fold.rs         # Folding/unfolding  
│   │   │   └── todo.rs         # Task management  
│   │   │  
│   │   ├── export/             # Export to different formats  
│   │   │   ├── mod.rs  
│   │   │   ├── html.rs  
│   │   │   ├── markdown.rs  
│   │   │   ├── pdf.rs  
│   │   │   └── plain.rs  
│   │   │  
│   │   └── agenda/             # Agenda view  
│   │       ├── mod.rs  
│   │       ├── query.rs        # Query language for filtering  
│   │       ├── view.rs         # Agenda view generation  
│   │       └── calendar.rs     # Calendar integration  
│   │  
│   ├── editor/                 # Editor functionality  
│   │   ├── mod.rs  
│   │   ├── buffer.rs           # Text buffer management  
│   │   ├── history.rs          # Undo/redo  
│   │   ├── cursor.rs           # Cursor/selection  
│   │   ├── view.rs             # View representation  
│   │   ├── renderer.rs         # Abstract renderer interface  
│   │   └── keybindings.rs      # Keyboard handling  
│   │  
│   ├── knowledge/              # Knowledge management (Obsidian-like features)  
│   │   ├── mod.rs  
│   │   ├── graph.rs            # Knowledge graph  
│   │   ├── backlinks.rs        # Backlink tracking  
│   │   ├── tags.rs             # Tag management and queries  
│   │   ├── search.rs           # Full-text search  
│   │   └── embeddings.rs       # Vector embeddings for semantic search  
│   │  
│   ├── ui/                     # UI components  
│   │   ├── mod.rs  
│   │   ├── common/             # Shared UI code  
│   │   │   ├── mod.rs  
│   │   │   ├── theme.rs        # Theming support  
│   │   │   ├── widgets.rs      # Common widgets  
│   │   │   └── layout.rs       # Layout management  
│   │   │  
│   │   ├── terminal/           # Terminal UI  
│   │   │   ├── mod.rs  
│   │   │   ├── app.rs          # TUI application  
│   │   │   ├── render.rs       # Terminal rendering  
│   │   │   ├── input.rs        # Input handling  
│   │   │   ├── views/          # Different view implementations  
│   │   │   └── widgets/        # TUI-specific widgets  
│   │   │  
│   │   └── gui/                # Graphical UI  
│   │       ├── mod.rs  
│   │       ├── app.rs          # GUI application  
│   │       ├── render.rs       # GUI rendering  
│   │       ├── input.rs        # Input handling  
│   │       ├── views/          # Different view implementations  
│   │       └── widgets/        # GUI-specific widgets  
│   │  
│   ├── sync/                   # Synchronization capabilities  
│   │   ├── mod.rs  
│   │   ├── git.rs              # Git integration  
│   │   ├── dropbox.rs          # Dropbox sync  
│   │   ├── webdav.rs           # WebDAV support  
│   │   └── local.rs            # Local file sync  
│   │  
│   └── utils/                  # Utility functions  
│       ├── mod.rs  
│       ├── error.rs            # Error handling  
│       ├── logging.rs          # Logging system  
│       └── fs.rs               # File system utilities  
│  
├── plugins/                    # Built-in plugins  
│   ├── mod.rs  
│   ├── calendar/               # Calendar integration  
│   ├── markdown/               # Markdown import/export  
│   ├── tasks/                  # Enhanced task management  
│   └── templates/              # Templates for notes  
│  
└── tests/                      # Integration tests  
    ├── parser_tests.rs  
    ├── model_tests.rs  
    ├── operations_tests.rs  
    └── fixtures/               # Test files  

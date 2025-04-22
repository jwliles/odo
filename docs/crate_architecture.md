# NeoOrg Crate Architecture

## Overview

This document outlines the proposed crate architecture for NeoOrg, focusing on creating a reusable editor core that can be embedded in various applications and UIs, similar to CodeMirror, Scintilla, or Monaco.

## Goals

- Create a modular, reusable editor core with no UI dependencies
- Provide clean API boundaries between components
- Support multiple frontends (terminal, GUI, web, etc.)
- Enable parallel development of features across different interfaces
- Make the editor embeddable in other Rust applications
- Ensure good testability of core functionality

## Proposed Architecture

```
neo-org-workspace/
├── Cargo.toml (workspace definition)
│
├── neo-org-core/              # Core document model & primitives
│   ├── Cargo.toml
│   └── src/
│       ├── document.rs        # Document model
│       ├── position.rs        # Cursor/position abstractions
│       ├── row.rs             # Line/row implementation
│       ├── search.rs          # Search functionality
│       ├── highlighting.rs    # Syntax highlighting
│       ├── filetype.rs        # File type detection
│       └── lib.rs             # Exports and tests
│
├── neo-org-editor/            # Editor logic and command handling
│   ├── Cargo.toml
│   └── src/
│       ├── command/           # Command handling
│       │   ├── mod.rs
│       │   ├── state.rs       # Command state management
│       │   ├── operator.rs    # Operators (delete, yank, etc.)
│       │   ├── motion.rs      # Motions (h, j, k, l, etc.)
│       │   └── text_object.rs # Text objects (word, line, etc.)
│       ├── mode.rs            # Editor modes (Normal, Insert, etc.)
│       ├── status.rs          # Status message handling
│       ├── history.rs         # Undo/redo functionality
│       ├── keymap.rs          # Key mapping infrastructure
│       ├── org/               # Org-specific editor functionality
│       │   ├── mod.rs
│       │   ├── element.rs     # Org element detection
│       │   ├── navigation.rs  # Org-specific navigation
│       │   └── operations.rs  # Org-specific operations
│       └── lib.rs             # Exports and tests
│
├── neo-org-terminal/          # Terminal UI implementation
│   ├── Cargo.toml
│   └── src/
│       ├── terminal.rs        # Terminal handling
│       ├── renderer.rs        # Terminal rendering
│       ├── input.rs           # Terminal input handling
│       └── lib.rs             # Exports and main implementation
│
├── neo-org-gui/               # GUI implementation
│   ├── Cargo.toml  
│   └── src/
│       ├── app.rs             # eframe app implementation
│       ├── renderer.rs        # GUI rendering
│       ├── widgets/           # Custom widgets
│       └── lib.rs             # Exports and main implementation
│
└── neo-org/                   # Main binary
    ├── Cargo.toml
    └── src/
        └── main.rs            # Entry point, CLI handling
```

## Key Interfaces

### `EditorCore` Trait (in neo-org-editor)

The core interface that frontends would implement:

```rust
pub trait EditorCore {
    /// Get the current document
    fn document(&self) -> &Document;
    
    /// Get mutable access to the document
    fn document_mut(&mut self) -> &mut Document;
    
    /// Get the current cursor position
    fn cursor_position(&self) -> Position;
    
    /// Set the cursor position
    fn set_cursor_position(&mut self, position: Position);
    
    /// Get the current editor mode
    fn mode(&self) -> Mode;
    
    /// Set the editor mode
    fn set_mode(&mut self, mode: Mode);
    
    /// Process a key press
    fn process_keypress(&mut self, key: KeyEvent) -> Result<(), EditorError>;
    
    /// Get the command state
    fn command_state(&self) -> &CommandState;
    
    /// Get mutable access to the command state
    fn command_state_mut(&mut self) -> &mut CommandState;
    
    /// Execute a command by name
    fn execute_command(&mut self, command: &str) -> Result<(), EditorError>;
}
```

### `EditorView` Trait (in neo-org-core)

The interface for rendering views of the editor:

```rust
pub trait EditorView {
    /// Initialize the view
    fn init(&mut self) -> Result<(), ViewError>;
    
    /// Render the editor state
    fn render(&mut self, editor: &dyn EditorCore) -> Result<(), ViewError>;
    
    /// Handle resize events
    fn resize(&mut self, width: u16, height: u16) -> Result<(), ViewError>;
    
    /// Clean up resources
    fn cleanup(&mut self) -> Result<(), ViewError>;
}
```

## Workflow Example

Here's how these components would work together:

```rust
// In a terminal application
fn main() -> Result<(), Box<dyn Error>> {
    // Create the editor core
    let mut editor = DefaultEditor::new();
    
    // Create the terminal view
    let mut terminal_view = TerminalView::new()?;
    
    // Main loop
    loop {
        // Render the editor
        terminal_view.render(&editor)?;
        
        // Handle input
        if let Some(key) = terminal_view.read_key()? {
            // Process the key in the editor
            editor.process_keypress(key)?;
            
            // Check for quit command
            if editor.should_quit() {
                break;
            }
        }
    }
    
    // Clean up
    terminal_view.cleanup()?;
    
    Ok(())
}
```

## Benefits for External Usage

1. **Embeddable**: The editor core can be embedded in any Rust application
2. **Customizable**: Applications can provide custom key mappings, commands, and UI
3. **Framework Agnostic**: Works with any UI framework (GTK, Qt, Tauri, egui, etc.)
4. **Extensible**: Plugin system can be added for custom functionality
5. **Testable**: Core functionality can be tested without UI dependencies

## Implementation Strategy

1. **Phase 1**: Extract core document model to neo-org-core
2. **Phase 2**: Extract editor logic to neo-org-editor
3. **Phase 3**: Create terminal and GUI frontends
4. **Phase 4**: Update main binary to use the new architecture
5. **Phase 5**: Publish crates to crates.io

## API Design Principles

1. **Minimal Dependencies**: Core should have minimal external dependencies
2. **Clear Boundaries**: APIs should have clear responsibilities
3. **Error Handling**: All errors should be propagated properly
4. **Documentation**: All public APIs should be well-documented
5. **Testing**: Comprehensive unit tests for all components
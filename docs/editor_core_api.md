# NeoOrg Editor Core API

This document outlines the proposed API for the neo-org-editor crate, which would be the primary reusable component that other applications can embed.

## Overview

The editor core provides a document model, editing capabilities, and command handling without any dependencies on specific UI frameworks. This makes it embeddable in any Rust application that needs a powerful text editor.

## Core Traits

### `EditorCore`

The primary trait that represents an editor instance:

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
    
    /// Returns true if the editor should quit
    fn should_quit(&self) -> bool;
    
    /// Get the current selection, if any
    fn selection(&self) -> Option<Selection>;
    
    /// Get the current status message
    fn status_message(&self) -> &str;
    
    /// Set the status message
    fn set_status_message(&mut self, message: &str);
    
    /// Get editor events since the last call
    fn take_events(&mut self) -> Vec<EditorEvent>;
    
    /// Undo the last action
    fn undo(&mut self) -> Result<(), EditorError>;
    
    /// Redo the last undone action
    fn redo(&mut self) -> Result<(), EditorError>;
}
```

### `DocumentProvider`

A trait for sources of document content:

```rust
pub trait DocumentProvider {
    /// Get the document content
    fn get_content(&self) -> Result<String, EditorError>;
    
    /// Save the document content
    fn save_content(&mut self, content: &str) -> Result<(), EditorError>;
    
    /// Get the file path, if any
    fn file_path(&self) -> Option<&Path>;
    
    /// Get the file name, if any
    fn file_name(&self) -> Option<&str>;
    
    /// Get the file type
    fn file_type(&self) -> FileType;
}
```

### `CommandHandler`

A trait for processing commands:

```rust
pub trait CommandHandler {
    /// Process a command
    fn handle_command(&mut self, editor: &mut dyn EditorCore, command: &str, args: &[&str]) -> Result<(), EditorError>;
    
    /// Get all available commands
    fn get_commands(&self) -> Vec<&str>;
    
    /// Get help for a command
    fn get_command_help(&self, command: &str) -> Option<String>;
}
```

## Key Structs

### `DefaultEditor`

The default implementation of the `EditorCore` trait:

```rust
pub struct DefaultEditor {
    document: Document,
    cursor_position: Position,
    mode: Mode,
    command_state: CommandState,
    should_quit: bool,
    selection: Option<Selection>,
    status_message: String,
    events: Vec<EditorEvent>,
    history: History,
    command_handlers: Vec<Box<dyn CommandHandler>>,
}

impl DefaultEditor {
    /// Create a new editor with a blank document
    pub fn new() -> Self { /* ... */ }
    
    /// Create a new editor with the given document
    pub fn new_with_document(document: Document) -> Self { /* ... */ }
    
    /// Create a builder for more complex initialization
    pub fn builder() -> DefaultEditorBuilder { /* ... */ }
    
    /// Register a command handler
    pub fn register_command_handler(&mut self, handler: Box<dyn CommandHandler>) { /* ... */ }
}
```

### `DefaultEditorBuilder`

A builder for creating customized `DefaultEditor` instances:

```rust
pub struct DefaultEditorBuilder {
    document: Option<Document>,
    mode: Mode,
    cursor_position: Position,
    syntax_highlighting: bool,
    command_handlers: Vec<Box<dyn CommandHandler>>,
}

impl DefaultEditorBuilder {
    /// Set the document
    pub fn with_document(mut self, document: Document) -> Self { /* ... */ }
    
    /// Set the initial mode
    pub fn with_mode(mut self, mode: Mode) -> Self { /* ... */ }
    
    /// Set the initial cursor position
    pub fn with_cursor_position(mut self, position: Position) -> Self { /* ... */ }
    
    /// Enable or disable syntax highlighting
    pub fn with_syntax_highlighting(mut self, enabled: bool) -> Self { /* ... */ }
    
    /// Add a command handler
    pub fn with_command_handler(mut self, handler: Box<dyn CommandHandler>) -> Self { /* ... */ }
    
    /// Build the editor
    pub fn build(self) -> DefaultEditor { /* ... */ }
}
```

### `Selection`

Represents a text selection:

```rust
pub struct Selection {
    start: Position,
    end: Position,
    mode: SelectionMode,
}

pub enum SelectionMode {
    Character,
    Line,
    Block,
}
```

### `EditorEvent`

Events that the editor emits:

```rust
pub enum EditorEvent {
    ModeChanged(Mode),
    CursorMoved(Position),
    DocumentChanged,
    SelectionChanged(Option<Selection>),
    CommandExecuted(String),
    StatusChanged(String),
    QuitRequested,
}
```

## Command System

The editor core would implement a flexible command system:

```rust
// Define command types
pub enum CommandType {
    Motion(Motion),
    Operator(Operator),
    TextObject(TextObject),
    Action,
}

// Register built-in commands
let mut editor = DefaultEditor::new();
editor.register_command_handler(Box::new(MotionCommandHandler::new()));
editor.register_command_handler(Box::new(OperatorCommandHandler::new()));
editor.register_command_handler(Box::new(OrgModeCommandHandler::new()));

// Execute a command
editor.execute_command("move_down").unwrap();
editor.execute_command("delete_line").unwrap();
```

## Input Processing

The editor would provide clean input processing:

```rust
// Process a keypress
editor.process_keypress(KeyEvent::Char('j')).unwrap();

// Process a key sequence
for key in &[KeyEvent::Char('d'), KeyEvent::Char('d')] {
    editor.process_keypress(*key).unwrap();
}
```

## Org-mode Integration

The editor would have special support for Org-mode:

```rust
// Get an Org-mode element at the cursor
let element = editor.org_element_at_cursor();

// Navigate to the next heading
editor.execute_command("org_next_heading").unwrap();

// Cycle a heading's TODO state
editor.execute_command("org_cycle_todo").unwrap();
```

## Extension System

The editor would support extensions:

```rust
// Register an extension
editor.register_extension(Box::new(MyCustomExtension::new()));

// Execute an extension command
editor.execute_command("my_extension.my_command").unwrap();
```

## Usage Example

```rust
use neo_org_core::{Document, Position};
use neo_org_editor::{DefaultEditor, EditorCore, KeyEvent, Mode};

// Create an editor
let mut editor = DefaultEditor::builder()
    .with_document(Document::from_string("Hello, world!"))
    .with_mode(Mode::Normal)
    .with_cursor_position(Position::new(0, 0))
    .with_syntax_highlighting(true)
    .build();

// Process key presses
editor.process_keypress(KeyEvent::Char('i')).unwrap();
assert_eq!(editor.mode(), Mode::Insert);

// Insert text
editor.process_keypress(KeyEvent::Char('H')).unwrap();
editor.process_keypress(KeyEvent::Char('i')).unwrap();
editor.process_keypress(KeyEvent::Char('!')).unwrap();

// Return to normal mode
editor.process_keypress(KeyEvent::Esc).unwrap();
assert_eq!(editor.mode(), Mode::Normal);

// Get the document content
let content = editor.document().to_string();
assert_eq!(content, "Hi!Hello, world!");
```

## API Design Principles

1. **Minimal**: Focused on essential functionality
2. **Extensible**: Easy to extend with custom behavior
3. **Trait-based**: Key components defined as traits for flexibility
4. **Error handling**: Comprehensive error handling with custom error types
5. **Event-driven**: Events for UI updates
6. **Stateful**: Maintains editor state
7. **Testable**: Easy to test in isolation
# Odo Crate Refactoring Plan

This document outlines the step-by-step plan to refactor Odo into a modular crate architecture that enables reuse in other projects. 

## Phase 1: Create Workspace Structure

1. Convert the current project to a workspace by:
   - Creating a top-level Cargo.toml workspace definition
   - Creating subdirectories for each planned crate
   - Moving appropriate code into each crate

```toml
# Top-level Cargo.toml
[workspace]
members = [
    "neo-org-core",
    "neo-org-editor",
    "neo-org-terminal",
    "neo-org-gui",
    "neo-org",
]
```

## Phase 2: Extract neo-org-core

1. Create the crate structure:
   ```
   neo-org-core/
   ├── Cargo.toml
   └── src/
       ├── lib.rs
       ├── document.rs
       ├── position.rs
       ├── row.rs
       ├── search.rs
       ├── highlighting.rs
       └── filetype.rs
   ```

2. Move existing code from `src/core/` to the new crate
3. Define clean interfaces for the document model
4. Create unit tests for core functionality
5. Update imports in the main project to use the new crate

## Phase 3: Extract neo-org-editor

1. Create the crate structure:
   ```
   neo-org-editor/
   ├── Cargo.toml
   └── src/
       ├── lib.rs
       ├── command/
       │   ├── mod.rs
       │   ├── state.rs
       │   ├── operator.rs
       │   ├── motion.rs
       │   └── text_object.rs
       ├── mode.rs
       ├── status.rs
       ├── history.rs
       ├── keymap.rs
       └── org/
           ├── mod.rs
           ├── element.rs
           ├── navigation.rs
           └── operations.rs
   ```

2. Move existing code from `src/editor/` to the new crate
3. Define the `EditorCore` trait as the main interface
4. Implement default behaviors
5. Create unit tests for editor functionality
6. Update imports in the main project

## Phase 4: Create UI Crates

### neo-org-terminal

1. Create the crate structure:
   ```
   neo-org-terminal/
   ├── Cargo.toml
   └── src/
       ├── lib.rs
       ├── terminal.rs
       ├── renderer.rs
       └── input.rs
   ```

2. Move terminal-specific code from `src/ui/terminal/` to the new crate
3. Implement the `EditorView` trait
4. Create a `TerminalEditor` that combines `EditorCore` with terminal UI
5. Update imports in the main project

### neo-org-gui

1. Create the crate structure:
   ```
   neo-org-gui/
   ├── Cargo.toml
   └── src/
       ├── lib.rs
       ├── app.rs
       ├── renderer.rs
       └── widgets/
   ```

2. Move GUI-specific code from `src/ui/gui/` to the new crate
3. Implement the `EditorView` trait for GUI
4. Create a `GuiEditor` that combines `EditorCore` with GUI
5. Update imports in the main project

## Phase 5: Update Main Binary

1. Create a minimal binary crate:
   ```
   neo-org/
   ├── Cargo.toml
   └── src/
       └── main.rs
   ```

2. Update main.rs to use the new crates
3. Implement CLI argument handling
4. Create appropriate editor instance based on arguments

## Phase 6: Documentation and Examples

1. Create comprehensive documentation for each crate
2. Add examples showing how to:
   - Embed the editor in another application
   - Create a custom frontend
   - Add custom commands
   - Extend the editor with plugins

## Phase 7: Create Integration Tests

1. Create integration tests that cover:
   - Document editing
   - Command processing
   - UI rendering
   - Cross-crate interactions

## Phase 8: Publish to crates.io

1. Finalize version numbers and dependencies
2. Create appropriate README.md files for each crate
3. Add license files
4. Publish to crates.io

## Implementation Timeline

| Phase | Estimated Duration | Dependencies |
|-------|---------------------|-------------|
| 1     | 1 day               | None        |
| 2     | 3 days              | Phase 1     |
| 3     | 5 days              | Phase 2     |
| 4     | 7 days              | Phase 3     |
| 5     | 2 days              | Phase 4     |
| 6     | 3 days              | All previous |
| 7     | 3 days              | All previous |
| 8     | 1 day               | All previous |

Total estimated time: ~25 days of development work

## Risks and Mitigation

1. **Risk**: Breaking existing functionality during refactoring
   - **Mitigation**: Create comprehensive tests before refactoring
   
2. **Risk**: API design choices that limit future flexibility
   - **Mitigation**: Review API designs carefully, consider extensibility
   
3. **Risk**: Increased complexity for simple use cases
   - **Mitigation**: Provide simplified wrappers for common scenarios
   
4. **Risk**: Dependency issues between crates
   - **Mitigation**: Minimize cross-dependencies, use clear interfaces

## Example Usage After Refactoring

```rust
use neo_org_core::Document;
use neo_org_editor::{DefaultEditor, EditorCore};
use neo_org_terminal::TerminalView;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document
    let document = Document::new();
    
    // Create an editor with that document
    let mut editor = DefaultEditor::new_with_document(document);
    
    // Create a terminal view
    let mut terminal_view = TerminalView::new()?;
    
    // Run the editor
    terminal_view.run(&mut editor)?;
    
    Ok(())
}
```

Or embedding in another application:

```rust
use neo_org_core::Document;
use neo_org_editor::{DefaultEditor, EditorCore, Mode};
use my_application::CustomView;

fn create_embedded_editor() -> impl EditorCore {
    // Create a document
    let document = Document::new();
    
    // Create an editor with custom settings
    let mut editor = DefaultEditor::builder()
        .with_document(document)
        .with_mode(Mode::Normal)
        .with_syntax_highlighting(true)
        .build();
        
    // Return the editor for use in the application
    editor
}
```
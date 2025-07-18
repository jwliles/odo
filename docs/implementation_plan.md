# Odo Implementation Plan

## Overview

This document outlines the implementation plan for enhancing Odo with Vim-inspired Org-mode-optimized motions and commands. The implementation follows a phased approach:

1. **Phase 0: Standard Vim Motions** - Implement core Vim navigation
2. **Phase 1: Org Structure-Aware Commands** - Add Org-aware enhancements 
3. **Phase 2: Org-specific Operations** - Implement specialized Org commands

## Architecture Changes

To implement the planned Vim-like features, we need to make the following architecture changes:

### 1. Enhanced Mode System

Update `src/editor/mode.rs` to support additional Vim-like modes:

```rust
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mode {
    Normal,    // Renamed from Command, standard Vim navigation mode
    Insert,    // Remains the same
    Visual,    // For text selection
    VisualLine, // For line-based selection
    Command,   // For : commands
}
```

### 2. Command State Management

Create a new module `src/editor/command.rs` to handle multi-key commands:

```rust
pub struct CommandState {
    // Track the current command being built
    buffer: Vec<char>,
    // Track if we're in an operator-pending state
    operator_pending: bool,
    // Current operator (if any)
    current_operator: Option<char>,
    // Count prefix for commands
    count: Option<usize>,
}

impl CommandState {
    // Methods for managing command state
    pub fn new() -> Self { /* ... */ }
    pub fn push(&mut self, c: char) { /* ... */ }
    pub fn clear(&mut self) { /* ... */ }
    pub fn is_operator_pending(&self) -> bool { /* ... */ }
    // etc.
}
```

### 3. Document Structure Recognition

Enhance `src/core/document.rs` with Org structure detection methods:

```rust
impl Document {
    // Existing methods...
    
    // New Org-specific methods
    pub fn is_heading(&self, y: usize) -> bool { /* ... */ }
    pub fn heading_level(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn next_heading(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn prev_heading(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn next_heading_same_level(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn prev_heading_same_level(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn parent_heading(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn is_list_item(&self, y: usize) -> bool { /* ... */ }
    pub fn next_list_item(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn prev_list_item(&self, y: usize) -> Option<usize> { /* ... */ }
    pub fn is_code_block(&self, y: usize) -> bool { /* ... */ }
    pub fn code_block_bounds(&self, y: usize) -> Option<(usize, usize)> { /* ... */ }
    pub fn is_todo_item(&self, y: usize) -> bool { /* ... */ }
    pub fn next_todo_item(&self, y: usize) -> Option<usize> { /* ... */ }
}
```

### 4. Updated TerminalEditor with Command Handling

Enhance `src/ui/terminal/terminal_editor.rs` with support for the new command system:

```rust
pub struct TerminalEditor {
    // Existing fields...
    command_state: CommandState,
    // Visual mode selection
    selection_start: Option<Position>,
}

impl TerminalEditor {
    // Existing methods...
    
    // Enhanced process_keypress to handle command state
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // Command processing including new state management
        // ...
    }
    
    // Methods for handling specific command types
    fn handle_motion(&mut self, motion: char) -> Result<(), std::io::Error> { /* ... */ }
    fn handle_operator(&mut self, operator: char) -> Result<(), std::io::Error> { /* ... */ }
    fn handle_text_object(&mut self, text_object: char) -> Result<(), std::io::Error> { /* ... */ }
    
    // Org-specific command handlers for g prefix
    fn handle_g_command(&mut self) -> Result<(), std::io::Error> { /* ... */ }
    
    // Enhanced move_cursor for Vim-style movements
    fn move_cursor(&mut self, key: Key, count: usize) { /* ... */ }
}
```

## Implementation Strategy

### Phase 0: Standard Vim Motions

1. Update `Mode` enum to support the new modes
2. Implement `CommandState` for managing multi-key commands
3. Implement basic motions (h,j,k,l,w,b,e,0,^,$,gg,G)
4. Implement basic operators (d,c,y) with motions
5. Implement text objects (word, line)
6. Add visual mode selection

### Phase 1: Org Structure-Aware Commands

1. Implement document structure recognition methods
2. Enhance existing motions to be Org-aware
3. Implement structure-aware folding
4. Implement structure-aware indentation

### Phase 2: Org-specific Operations

1. Implement g-prefix commands for Org navigation
2. Add heading/list manipulation commands
3. Implement TODO state cycling
4. Add code block manipulation

## Testing Strategy

For each phase:

1. Create unit tests for the new command handling logic
2. Add integration tests with sample Org files
3. Test edge cases (empty documents, large files, long lines)
4. Visual verification of highlighting and UI elements

## UI Considerations

1. Update status line to show current mode
2. Add visual indicators for selections
3. Implement highlighting for the current heading/section
4. Show folding indicators for collapsed sections
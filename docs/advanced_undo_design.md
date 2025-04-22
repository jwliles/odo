# Advanced Undo/Redo System Design

This document outlines the design for a sophisticated undo/redo system in NeoOrg, inspired by Pijul's patch theory, Tree-sitter's structural understanding, Vim's persistent undo tree, and Emacs' kill ring.

## Design Goals

1. **Structural Understanding**: Use Tree-sitter to understand document structure beyond just text
2. **Patch-Based History**: Track changes as patches rather than snapshots
3. **Non-Linear History**: Support branching undo history like Vim's undo tree
4. **Persistent Storage**: Save undo history across editing sessions
5. **Context-Aware Operations**: Intelligently handle operations based on document structure
6. **Advanced Clipboard**: Implement an Emacs-style kill ring with multiple entries
7. **Region Operations**: Support region-based undo operations

## Architecture Components

### 1. `HistoryNode` Struct

Represents a node in the undo tree:

```rust
pub struct HistoryNode {
    /// The unique identifier for this node
    id: u64,
    /// The patch that transforms the document from the parent state to this state
    patch: Patch,
    /// The parent node in the history tree
    parent: Option<u64>,
    /// Child nodes in the history tree
    children: Vec<u64>,
    /// Timestamp when this change was made
    timestamp: DateTime<Utc>,
    /// Description of the change (for UI)
    description: String,
    /// Was this an automated change? (e.g., auto-formatting)
    is_automated: bool,
    /// Cursor position after the change
    cursor_position: Position,
}
```

### 2. `Patch` Struct

Represents a document transformation:

```rust
pub struct Patch {
    /// The operations that make up this patch
    operations: Vec<Operation>,
    /// Structural context from Tree-sitter for this patch
    context: StructuralContext,
    /// Is this patch reversible?
    is_reversible: bool,
    /// The inverse patch (for undo)
    inverse: Option<Box<Patch>>,
}
```

### 3. `Operation` Enum

Represents a single edit operation:

```rust
pub enum Operation {
    /// Insert text at a position
    Insert { position: Position, text: String },
    /// Delete text in a range
    Delete { range: Range, text: String },
    /// Replace text in a range
    Replace { range: Range, old_text: String, new_text: String },
    /// Move text from one range to another
    Move { source_range: Range, destination: Position },
    /// Structural operation (e.g., toggle heading level, cycle TODO state)
    Structural { kind: StructuralOperationKind, node_id: NodeId, data: StructuralOperationData },
}
```

### 4. `StructuralContext` Struct

Captures the Tree-sitter structural context:

```rust
pub struct StructuralContext {
    /// The Tree-sitter node type at the edit position
    node_type: String,
    /// The path to the node in the syntax tree
    node_path: Vec<String>,
    /// Tree-sitter node ID for reference
    node_id: NodeId,
    /// Additional semantic information
    semantic_info: HashMap<String, String>,
}
```

### 5. `HistoryTree` Struct

Manages the entire undo/redo history:

```rust
pub struct HistoryTree {
    /// All history nodes, indexed by ID
    nodes: HashMap<u64, HistoryNode>,
    /// Current active node ID
    current_node: u64,
    /// Root node ID
    root_node: u64,
    /// Maximum number of nodes to keep in memory
    max_nodes: usize,
    /// File path for persistent storage
    storage_path: Option<PathBuf>,
}

impl HistoryTree {
    /// Create a new history tree
    pub fn new() -> Self { /* ... */ }
    
    /// Apply a new change to the document
    pub fn apply_change(&mut self, patch: Patch, description: String) -> u64 { /* ... */ }
    
    /// Undo the last change
    pub fn undo(&mut self) -> Option<&Patch> { /* ... */ }
    
    /// Redo the last undone change
    pub fn redo(&mut self) -> Option<&Patch> { /* ... */ }
    
    /// Navigate to a specific node in the history tree
    pub fn navigate_to_node(&mut self, node_id: u64) -> Option<&Patch> { /* ... */ }
    
    /// Save the history tree to disk
    pub fn save(&self) -> Result<(), HistoryError> { /* ... */ }
    
    /// Load the history tree from disk
    pub fn load(path: &Path) -> Result<Self, HistoryError> { /* ... */ }
    
    /// Prune old history to stay within max_nodes
    pub fn prune(&mut self) { /* ... */ }
    
    /// Get a visualization of the history tree (for UI)
    pub fn visualize(&self) -> HistoryVisualization { /* ... */ }
}
```

### 6. `KillRing` Struct

Implements an Emacs-style kill ring:

```rust
pub struct KillRing {
    /// The entries in the kill ring
    entries: VecDeque<KillEntry>,
    /// Current position in the kill ring
    current: usize,
    /// Maximum number of entries
    max_entries: usize,
}

pub struct KillEntry {
    /// The text in this entry
    text: String,
    /// Structural context at the time of killing
    context: StructuralContext,
    /// Timestamp when this was added
    timestamp: DateTime<Utc>,
    /// Was this a kill (cut) or copy?
    is_kill: bool,
}

impl KillRing {
    /// Create a new kill ring
    pub fn new(max_entries: usize) -> Self { /* ... */ }
    
    /// Add an entry to the kill ring
    pub fn add(&mut self, text: String, context: StructuralContext, is_kill: bool) { /* ... */ }
    
    /// Get the current entry
    pub fn current(&self) -> Option<&KillEntry> { /* ... */ }
    
    /// Rotate to the next entry
    pub fn rotate_next(&mut self) -> Option<&KillEntry> { /* ... */ }
    
    /// Rotate to the previous entry
    pub fn rotate_prev(&mut self) -> Option<&KillEntry> { /* ... */ }
    
    /// Clear the kill ring
    pub fn clear(&mut self) { /* ... */ }
    
    /// Save the kill ring to disk
    pub fn save(&self, path: &Path) -> Result<(), KillRingError> { /* ... */ }
    
    /// Load the kill ring from disk
    pub fn load(path: &Path) -> Result<Self, KillRingError> { /* ... */ }
}
```

### 7. Integration with Tree-sitter

```rust
pub struct TreeSitterIntegration {
    /// The Tree-sitter parser
    parser: Parser,
    /// Current syntax tree
    tree: Tree,
    /// Language definition
    language: Language,
}

impl TreeSitterIntegration {
    /// Create a new Tree-sitter integration
    pub fn new(language: Language) -> Self { /* ... */ }
    
    /// Update the syntax tree with the current document
    pub fn update(&mut self, text: &str) { /* ... */ }
    
    /// Get the node at a position
    pub fn node_at_position(&self, position: Position) -> Option<Node> { /* ... */ }
    
    /// Get the structural context at a position
    pub fn structural_context_at_position(&self, position: Position) -> StructuralContext { /* ... */ }
    
    /// Walk the tree and apply a function to each node
    pub fn walk<F>(&self, f: F) where F: Fn(&Node) { /* ... */ }
}
```

## Integration with Editor Core

```rust
pub struct EditorCore {
    // ... other fields ...
    
    /// Undo/redo history tree
    history: HistoryTree,
    /// Kill ring
    kill_ring: KillRing,
    /// Tree-sitter integration
    treesitter: TreeSitterIntegration,
}

impl EditorCore {
    // ... other methods ...
    
    /// Undo the last change
    pub fn undo(&mut self) -> Result<(), EditorError> {
        if let Some(patch) = self.history.undo() {
            self.apply_inverse_patch(patch)?;
            Ok(())
        } else {
            Err(EditorError::NothingToUndo)
        }
    }
    
    /// Redo the last undone change
    pub fn redo(&mut self) -> Result<(), EditorError> {
        if let Some(patch) = self.history.redo() {
            self.apply_patch(patch)?;
            Ok(())
        } else {
            Err(EditorError::NothingToRedo)
        }
    }
    
    /// Apply an edit to the document and record it in the history
    pub fn edit(&mut self, operation: Operation, description: String) -> Result<(), EditorError> {
        // Get structural context using Tree-sitter
        let context = self.treesitter.structural_context_at_position(self.cursor_position);
        
        // Create a patch with the operation and context
        let patch = Patch {
            operations: vec![operation],
            context,
            is_reversible: true,
            inverse: None, // Will be filled in during application
        };
        
        // Apply the patch and create its inverse
        let (applied_patch, _) = self.apply_patch_with_inverse(patch)?;
        
        // Record in history
        self.history.apply_change(applied_patch, description);
        
        Ok(())
    }
    
    /// Kill (cut) text to the kill ring
    pub fn kill_text(&mut self, range: Range) -> Result<(), EditorError> {
        // Get the text
        let text = self.document.text_in_range(&range)?;
        
        // Get structural context
        let context = self.treesitter.structural_context_at_position(range.start);
        
        // Add to kill ring
        self.kill_ring.add(text.clone(), context, true);
        
        // Delete from document
        self.edit(
            Operation::Delete { range, text },
            "Kill text".to_string(),
        )
    }
    
    /// Yank (paste) text from the kill ring
    pub fn yank(&mut self) -> Result<(), EditorError> {
        if let Some(entry) = self.kill_ring.current() {
            self.edit(
                Operation::Insert { 
                    position: self.cursor_position,
                    text: entry.text.clone(),
                },
                "Yank text".to_string(),
            )
        } else {
            Err(EditorError::EmptyKillRing)
        }
    }
    
    /// Yank-pop (cycle through kill ring after yanking)
    pub fn yank_pop(&mut self) -> Result<(), EditorError> {
        // First undo the previous yank
        self.undo()?;
        
        // Rotate to next kill ring entry
        if let Some(entry) = self.kill_ring.rotate_next() {
            self.edit(
                Operation::Insert {
                    position: self.cursor_position,
                    text: entry.text.clone(),
                },
                "Yank pop".to_string(),
            )
        } else {
            Err(EditorError::EmptyKillRing)
        }
    }
}
```

## Persistent Storage

### 1. History File Format

The undo history will be stored in a JSON file with the following format:

```json
{
  "version": 1,
  "root_node": 1,
  "current_node": 42,
  "nodes": [
    {
      "id": 1,
      "patch": { /* serialized patch */ },
      "parent": null,
      "children": [2, 3],
      "timestamp": "2023-04-21T15:30:00Z",
      "description": "Initial document",
      "is_automated": false,
      "cursor_position": { "line": 0, "column": 0 }
    },
    /* more nodes */
  ]
}
```

### 2. Kill Ring File Format

The kill ring will be stored in a JSON file with the following format:

```json
{
  "version": 1,
  "current": 0,
  "max_entries": 60,
  "entries": [
    {
      "text": "Kill ring entry text",
      "context": { /* serialized context */ },
      "timestamp": "2023-04-21T15:30:00Z",
      "is_kill": true
    },
    /* more entries */
  ]
}
```

## UI Integration

### 1. Undo Tree Visualization

For the GUI, we'll provide a visualization of the undo tree:

```rust
pub struct UndoTreeWidget {
    history: &'a HistoryTree,
    selected_node: Option<u64>,
}

impl UndoTreeWidget {
    /// Create a new undo tree widget
    pub fn new(history: &'a HistoryTree) -> Self { /* ... */ }
    
    /// Draw the widget using egui
    pub fn ui(&mut self, ui: &mut egui::Ui) { /* ... */ }
    
    /// Handle node selection
    pub fn on_node_selected(&mut self, node_id: u64) { /* ... */ }
}
```

### 2. Kill Ring UI

For the GUI, we'll provide a kill ring browser:

```rust
pub struct KillRingWidget {
    kill_ring: &'a KillRing,
    selected_entry: Option<usize>,
}

impl KillRingWidget {
    /// Create a new kill ring widget
    pub fn new(kill_ring: &'a KillRing) -> Self { /* ... */ }
    
    /// Draw the widget using egui
    pub fn ui(&mut self, ui: &mut egui::Ui) { /* ... */ }
    
    /// Handle entry selection
    pub fn on_entry_selected(&mut self, index: usize) { /* ... */ }
}
```

## Key Bindings

### Vim-Style Bindings

```
u           - Undo
Ctrl-r      - Redo
g u         - Navigate undo tree (open undo browser)
"[a-z]y     - Yank to named register
"[a-z]p     - Paste from named register
```

### Emacs-Style Bindings

```
C-/         - Undo
C-_         - Undo
C-y         - Yank (paste)
M-y         - Yank-pop (cycle through kill ring)
C-g C-/     - Undo visualizer
```

## Implementation Plan

1. **Phase 1**: Implement basic `HistoryTree` and `Patch` structures
2. **Phase 2**: Integrate Tree-sitter for structural understanding
3. **Phase 3**: Add persistent storage for undo history
4. **Phase 4**: Implement the kill ring functionality
5. **Phase 5**: Add UI visualizations for both terminal and GUI
6. **Phase 6**: Create commands and key bindings

## Benefits of This Approach

1. **Intelligent Undo/Redo**: Context-aware operations that understand document structure
2. **Non-Linear Editing**: Support for exploring different editing paths
3. **Advanced Clipboard**: Multiple clipboard entries with structural context
4. **Persistent History**: Undo/redo survives across editing sessions
5. **Structure-Aware**: Understands Org-mode elements for more natural operations
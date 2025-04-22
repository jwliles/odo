# Structural Undo Crate Design

This document outlines the design for `structural-undo`, an independent crate that provides advanced undo/redo functionality with structural awareness. This crate can be used in any Rust application that needs sophisticated undo/redo capabilities, including text editors, graphical editors, and specialized applications like recipe editors.

## Overview

The `structural-undo` crate provides a standalone implementation of a patch-based undo system with structural awareness. It integrates with Tree-sitter for syntax understanding but can also work with custom structural parsers for domain-specific applications.

## Crate Structure

```
structural-undo/
├── Cargo.toml
└── src/
    ├── lib.rs                 # Main exports and documentation
    ├── patch.rs               # Patch and Operation implementations
    ├── history.rs             # History tree implementation
    ├── context.rs             # Structural context abstractions
    ├── storage/               # Persistent storage
    │   ├── mod.rs
    │   ├── json.rs            # JSON serialization
    │   └── binary.rs          # Binary serialization
    ├── clipboard/             # Advanced clipboard functionality
    │   ├── mod.rs
    │   ├── kill_ring.rs       # Kill ring implementation
    │   └── registers.rs       # Named registers
    ├── treesitter/            # Tree-sitter integration (optional feature)
    │   ├── mod.rs
    │   └── integration.rs
    ├── visualization/         # Visualization utilities
    │   ├── mod.rs
    │   └── tree.rs            # Tree visualization
    └── error.rs               # Error types
```

## Core Components

### Patch System

The heart of the undo system is the patch representation of changes:

```rust
/// A patch representing a transformation of a document
pub struct Patch<T> {
    /// The operations that make up this patch
    pub operations: Vec<Operation<T>>,
    /// Structural context for this patch
    pub context: ContextData,
    /// Metadata about this patch
    pub metadata: PatchMetadata,
    /// The inverse patch (for undo)
    pub inverse: Option<Box<Patch<T>>>,
}

/// A single operation within a patch
pub enum Operation<T> {
    /// Insert content at a position
    Insert {
        /// The position to insert at
        position: Position,
        /// The content to insert
        content: T,
    },
    /// Delete content in a range
    Delete {
        /// The range to delete
        range: Range,
        /// The deleted content (for undo)
        content: T,
    },
    /// Replace content in a range
    Replace {
        /// The range to replace
        range: Range,
        /// The old content (for undo)
        old_content: T,
        /// The new content
        new_content: T,
    },
    /// Move content from one range to another
    Move {
        /// The source range
        source_range: Range,
        /// The destination position
        destination: Position,
        /// The moved content
        content: T,
    },
    /// Custom operation with application and inversion functions
    Custom {
        /// Type identifier for serialization
        type_id: String,
        /// Application function
        apply: Box<dyn Fn(&mut T) -> Result<(), PatchError>>,
        /// Inversion function
        invert: Box<dyn Fn(&T) -> Result<Operation<T>, PatchError>>,
        /// Serializable data for this operation
        data: Value,
    },
}
```

### History Tree

The non-linear history is managed by a tree structure:

```rust
/// A tree of history states
pub struct HistoryTree<T> {
    /// All nodes in the history tree
    nodes: HashMap<NodeId, HistoryNode<T>>,
    /// Current active node
    current_node: NodeId,
    /// Root node
    root_node: NodeId,
    /// Configuration options
    config: HistoryConfig,
}

/// A node in the history tree
pub struct HistoryNode<T> {
    /// Node identifier
    id: NodeId,
    /// The patch that led to this state
    patch: Option<Patch<T>>,
    /// Parent node identifier
    parent: Option<NodeId>,
    /// Child node identifiers
    children: Vec<NodeId>,
    /// Timestamp when this node was created
    timestamp: DateTime<Utc>,
    /// Description of the change
    description: String,
}
```

### Structural Context

The context system provides structural awareness:

```rust
/// Structural context data
pub struct ContextData {
    /// The structural path to the edited element
    path: Vec<PathElement>,
    /// Additional context data
    attributes: HashMap<String, Value>,
}

/// A path element in a structural path
pub struct PathElement {
    /// Element type
    element_type: String,
    /// Element identifier (if any)
    id: Option<String>,
    /// Index within parent (if applicable)
    index: Option<usize>,
    /// Additional attributes
    attributes: HashMap<String, Value>,
}
```

### Tree-sitter Integration

Optional integration with Tree-sitter for syntax understanding:

```rust
/// Tree-sitter integration for structural context
pub struct TreeSitterContext {
    /// Tree-sitter parser
    parser: Parser,
    /// Current syntax tree
    tree: Option<Tree>,
    /// Language definition
    language: Language,
}

impl TreeSitterContext {
    /// Create a new Tree-sitter context
    pub fn new(language: Language) -> Self { /* ... */ }
    
    /// Update the syntax tree
    pub fn update(&mut self, text: &str) -> Result<(), TreeSitterError> { /* ... */ }
    
    /// Get context data at a position
    pub fn context_at_position(&self, position: Position) -> Result<ContextData, TreeSitterError> { /* ... */ }
    
    /// Convert Tree-sitter node to path element
    fn node_to_path_element(&self, node: Node) -> PathElement { /* ... */ }
}
```

### Kill Ring

Advanced clipboard functionality:

```rust
/// A circular buffer of clipboard entries
pub struct KillRing<T> {
    /// The entries in the ring
    entries: VecDeque<KillEntry<T>>,
    /// Current position in the ring
    current: usize,
    /// Maximum number of entries
    max_entries: usize,
}

/// A single entry in the kill ring
pub struct KillEntry<T> {
    /// The content
    content: T,
    /// Context data
    context: ContextData,
    /// When this entry was created
    timestamp: DateTime<Utc>,
    /// Whether this was a kill (cut) or copy
    is_kill: bool,
}
```

## Feature Flags

The crate uses feature flags to make it adaptable to different use cases:

```toml
[features]
default = ["treesitter", "storage-json", "visualization"]
treesitter = ["dep:tree-sitter"]
storage-json = ["dep:serde", "dep:serde_json"]
storage-binary = ["dep:bincode"]
visualization = []
clipboard = []
```

## API Examples

### Basic Usage

```rust
use structural_undo::{HistoryTree, Patch, Operation, Position, Range};

// Create a history tree for a string document
let mut history = HistoryTree::<String>::new();

// Apply an edit
let mut document = String::from("Hello, world!");
let patch = Patch::new(vec![
    Operation::Replace {
        range: Range::new(Position::new(0, 7), Position::new(0, 12)),
        old_content: "world".to_string(),
        new_content: "everyone".to_string(),
    }
]);

// Apply the patch to document and record in history
history.apply_change(&mut document, patch, "Replace 'world' with 'everyone'".to_string());
assert_eq!(document, "Hello, everyone!");

// Undo the change
history.undo(&mut document).unwrap();
assert_eq!(document, "Hello, world!");

// Redo the change
history.redo(&mut document).unwrap();
assert_eq!(document, "Hello, everyone!");
```

### Tree-sitter Integration

```rust
use structural_undo::{HistoryTree, TreeSitterContext, Language};
use tree_sitter_org::language;

// Create a Tree-sitter context
let mut ts_context = TreeSitterContext::new(language());

// Update with document content
let document = "* Heading 1\n** Heading 2\nContent";
ts_context.update(document).unwrap();

// Get context at a position
let position = Position::new(1, 0); // Start of "** Heading 2"
let context = ts_context.context_at_position(position).unwrap();

// The context contains structural information
assert_eq!(context.path[0].element_type, "document");
assert_eq!(context.path[1].element_type, "heading");
assert_eq!(context.path[1].attributes.get("level").unwrap(), &Value::Integer(2));
```

### Persistent Storage

```rust
use structural_undo::{HistoryTree, storage::JsonStorage};
use std::path::Path;

// Save history to disk
let path = Path::new("history.json");
let storage = JsonStorage::new();
storage.save(&history, path).unwrap();

// Load history from disk
let loaded_history = storage.load::<String>(path).unwrap();
```

### Kill Ring

```rust
use structural_undo::clipboard::{KillRing, KillEntry};

// Create a kill ring
let mut kill_ring = KillRing::<String>::new(10);

// Add entries
kill_ring.add("First entry".to_string(), context1, true);
kill_ring.add("Second entry".to_string(), context2, true);

// Get current entry
let current = kill_ring.current().unwrap();
assert_eq!(current.content, "Second entry");

// Rotate to previous entry
kill_ring.rotate_prev();
let current = kill_ring.current().unwrap();
assert_eq!(current.content, "First entry");
```

## Recipe Editor Integration Example

Here's how this crate could be used in a recipe editor:

```rust
use structural_undo::{HistoryTree, Patch, Operation, ContextData, Position, Range};

// Define a Recipe type
struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}

struct Ingredient {
    name: String,
    quantity: f32,
    unit: String,
}

// Create a history tree for recipe documents
let mut history = HistoryTree::<Recipe>::new();

// Create a recipe
let mut recipe = Recipe {
    name: "Chocolate Cake".to_string(),
    ingredients: vec![
        Ingredient {
            name: "Flour".to_string(),
            quantity: 2.0,
            unit: "cups".to_string(),
        },
        // ...more ingredients
    ],
    steps: vec![
        "Preheat oven to 350°F".to_string(),
        // ...more steps
    ],
};

// Create a structural context
let mut context = ContextData::new();
context.path.push(PathElement::new("recipe"));
context.path.push(PathElement::new("ingredient")
    .with_attribute("index", 0));

// Create a patch to change an ingredient
let patch = Patch::new_with_context(vec![
    Operation::Replace {
        range: Range::new(Position::zero(), Position::zero()), // Not used for structured data
        old_content: recipe.ingredients[0].clone(),
        new_content: Ingredient {
            name: "Cake Flour".to_string(),
            quantity: 2.5,
            unit: "cups".to_string(),
        },
    }
], context);

// Define how to apply the patch
let apply_fn = |recipe: &mut Recipe, op: &Operation<Ingredient>| {
    match op {
        Operation::Replace { old_content: _, new_content, .. } => {
            recipe.ingredients[0] = new_content.clone();
            Ok(())
        }
        _ => Err(PatchError::UnsupportedOperation),
    }
};

// Apply the change and record in history
history.apply_custom_change(&mut recipe, patch, apply_fn, "Change flour to cake flour".to_string());

// The recipe has been updated
assert_eq!(recipe.ingredients[0].name, "Cake Flour");
assert_eq!(recipe.ingredients[0].quantity, 2.5);

// Undo the change
history.undo_custom(&mut recipe, |recipe, patch| {
    // Custom undo logic here
    match &patch.operations[0] {
        Operation::Replace { old_content, .. } => {
            recipe.ingredients[0] = old_content.clone();
            Ok(())
        }
        _ => Err(PatchError::UnsupportedOperation),
    }
}).unwrap();

// The recipe has been restored
assert_eq!(recipe.ingredients[0].name, "Flour");
assert_eq!(recipe.ingredients[0].quantity, 2.0);
```

## Benefits for Various Applications

### Text Editors

- Non-linear undo history
- Structural awareness for code/markup
- Persistent undo across sessions
- Advanced clipboard with multiple entries

### Recipe Editors

- Track changes to ingredients, quantities, steps
- Structural awareness of recipe components
- Undo/redo specific changes (e.g., "change flour quantity")
- Clipboard specialized for recipe elements

### Graphic Editors

- Track changes to shapes, colors, layers
- Group operations logically
- Branch exploration for design alternatives
- Structural awareness of the canvas hierarchy

## Implementation Plan

1. **Phase 1**: Core patch system implementation
2. **Phase 2**: History tree with non-linear navigation
3. **Phase 3**: Persistent storage (JSON and binary)
4. **Phase 4**: Tree-sitter integration
5. **Phase 5**: Kill ring and clipboard functionality
6. **Phase 6**: Visualization utilities
7. **Phase 7**: Examples and documentation
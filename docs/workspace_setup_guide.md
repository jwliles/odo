# Workspace Setup Guide

This document provides practical steps for setting up the Odo workspace and migrating the current codebase.

## Initial Workspace Setup

### 1. Create Workspace Structure

```bash
# Create directories for each crate
mkdir -p .cargo structural-undo/src neo-org-core/src neo-org-editor/src neo-org-terminal/src neo-org-gui/src neo-org/src
```

### 2. Create Root Cargo.toml

Create a workspace-level `Cargo.toml` file:

```toml
[workspace]
members = [
    "structural-undo",
    "neo-org-core",
    "neo-org-editor",
    "neo-org-terminal",
    "neo-org-gui",
    "neo-org",
]
resolver = "2"

[workspace.package]
version = "0.3.0"
authors = ["J. W. Liles <...>"]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/jwliles/Odo"

[workspace.dependencies]
# Core dependencies
tree-sitter = "0.20.10"
unicode-segmentation = "1.12.0"
# UI dependencies
crossterm = "0.28.1"
termion = "4.0.5"
egui = "0.25.0"
eframe = "0.25.0"
rfd = "0.13.0"
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# Utilities
regex = "1.10.3"
chrono = { version = "0.4", features = ["serde"] }
```

### 3. Set Up Workspace Config

Create `.cargo/config.toml`:

```toml
[alias]
# Workspace-wide aliases
check-all = "check --workspace"
test-all = "test --workspace"
docs = "doc --no-deps --open"
```

### 4. Set Up Initial Crates

#### structural-undo

Create `structural-undo/Cargo.toml`:

```toml
[package]
name = "structural-undo"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Advanced undo/redo system with structural awareness and Tree-sitter integration"

[dependencies]
# Core dependencies
serde = { workspace = true, optional = true }
serde_json = { workspace = true, optional = true }
chrono = { workspace = true }
# Optional dependencies
tree-sitter = { workspace = true, optional = true }

[features]
default = ["storage-json"]
treesitter = ["dep:tree-sitter"]
storage-json = ["dep:serde", "dep:serde_json"]
```

Create `structural-undo/src/lib.rs`:

```rust
//! # Structural Undo
//! 
//! Advanced undo/redo system with structural awareness and Tree-sitter integration.

mod patch;
mod history;
mod context;
mod error;

#[cfg(feature = "storage-json")]
pub mod storage;

#[cfg(feature = "treesitter")]
pub mod treesitter;

// Public exports
pub use patch::{Patch, Operation};
pub use history::{HistoryTree, HistoryNode, HistoryConfig};
pub use context::{ContextData, PathElement};
pub use error::{UndoError, PatchError};

```

#### neo-org-core

Create `neo-org-core/Cargo.toml`:

```toml
[package]
name = "neo-org-core"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Core document model for Odo editor"

[dependencies]
unicode-segmentation = { workspace = true }
tree-sitter = { workspace = true, optional = true }
structural-undo = { path = "../structural-undo" }
serde = { workspace = true, features = ["derive"], optional = true }
regex = { workspace = true }

[features]
default = ["serialization"]
serialization = ["dep:serde"]
treesitter = ["dep:tree-sitter"]
```

Create `neo-org-core/src/lib.rs`:

```rust
//! # Neo-Org Core
//!
//! Core document model for Odo editor.

mod document;
mod position;
mod row;
mod search;
mod highlighting;
mod filetype;

// Public exports
pub use document::Document;
pub use position::Position;
pub use row::Row;
pub use search::{SearchDirection, SearchResult};
pub use highlighting::Highlighting;
pub use filetype::FileType;
```

#### neo-org-editor

Create `neo-org-editor/Cargo.toml`:

```toml
[package]
name = "neo-org-editor"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Editor logic and command handling for Odo"

[dependencies]
neo-org-core = { path = "../neo-org-core" }
structural-undo = { path = "../structural-undo" }
serde = { workspace = true, features = ["derive"], optional = true }

[features]
default = ["serialization"]
serialization = ["dep:serde", "neo-org-core/serialization"]
```

Create `neo-org-editor/src/lib.rs`:

```rust
//! # Neo-Org Editor
//!
//! Editor logic and command handling for Odo.

mod mode;
mod command;
mod status;
mod keymap;

pub mod org;

// Public exports
pub use mode::Mode;
pub use command::{CommandState, TextObject, Operator, Motion};
pub use status::StatusMessage;
pub use keymap::KeyMap;
```

#### neo-org-terminal

Create `neo-org-terminal/Cargo.toml`:

```toml
[package]
name = "neo-org-terminal"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Terminal interface for Odo"

[dependencies]
neo-org-core = { path = "../neo-org-core" }
neo-org-editor = { path = "../neo-org-editor" }
crossterm = { workspace = true }
termion = { workspace = true }
```

Create `neo-org-terminal/src/lib.rs`:

```rust
//! # Neo-Org Terminal
//!
//! Terminal interface for Odo.

mod terminal;
mod renderer;
mod input;

// Public exports
pub use terminal::Terminal;
pub use renderer::TerminalRenderer;
pub use input::InputHandler;
```

#### neo-org-gui

Create `neo-org-gui/Cargo.toml`:

```toml
[package]
name = "neo-org-gui"
version = "0.1.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "GUI interface for Odo using egui"

[dependencies]
neo-org-core = { path = "../neo-org-core" }
neo-org-editor = { path = "../neo-org-editor" }
egui = { workspace = true }
eframe = { workspace = true }
rfd = { workspace = true }
```

Create `neo-org-gui/src/lib.rs`:

```rust
//! # Neo-Org GUI
//!
//! GUI interface for Odo using egui.

mod app;
mod renderer;
mod widgets;

// Public exports
pub use app::{OdoApp, AppConfig};
pub use renderer::GuiRenderer;
```

#### neo-org

Create `neo-org/Cargo.toml`:

```toml
[package]
name = "neo-org"
version = "0.3.0"
edition = "2021"
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Modern, modal text editor optimized for Org mode"

[dependencies]
neo-org-core = { path = "../neo-org-core" }
neo-org-editor = { path = "../neo-org-editor" }
neo-org-terminal = { path = "../neo-org-terminal" }
neo-org-gui = { path = "../neo-org-gui" }
structural-undo = { path = "../structural-undo" }
```

Create `neo-org/src/main.rs`:

```rust
//! # Neo-Org
//!
//! Modern, modal text editor optimized for Org mode.

use std::env;
use std::process;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Start the appropriate interface based on args
    println!("Odo workspace-based version starting...");
    
    // Placeholder until real implementation
    process::exit(0);
}
```

## Migrating Existing Code

### 1. Backup Current Code

```bash
git stash push -m "Backup before workspace migration"
```

### 2. Move Core Code

```bash
# Copy core files to neo-org-core
cp src/core/* neo-org-core/src/
```

### 3. Move Editor Code

```bash
# Copy editor files to neo-org-editor
cp src/editor/* neo-org-editor/src/
```

### 4. Move Terminal UI Code

```bash
# Copy terminal files to neo-org-terminal
cp src/ui/terminal/* neo-org-terminal/src/
```

### 5. Move GUI Code

```bash
# Copy GUI files to neo-org-gui
cp src/ui/gui/* neo-org-gui/src/
```

### 6. Update Paths and Imports

You'll need to update import paths in all files. For example:

From:
```rust
use crate::core::Document;
use crate::editor::Mode;
```

To:
```rust
use neo_org_core::Document;
use neo_org_editor::Mode;
```

### 7. Create Structural-Undo Implementation

Create the initial implementation based on the design document.

### 8. Initial Compilation Check

```bash
cargo check-all
```

### 9. Workspace-wide Testing

```bash
cargo test-all
```

## Phased Implementation Strategy

1. Start with minimal implementations of each crate
2. Focus on making everything compile and run
3. Gradually add features to each crate
4. Test integration between crates
5. Refine APIs as the implementation progresses

## Development Workflow

1. **Feature Branch per Component**:
   - Create feature branches for major components
   - For example: `feature/structural-undo`, `feature/core-refactor`

2. **Pull Request Flow**:
   - Create PRs for each component
   - Review for API design and implementation quality
   - Merge when ready

3. **Testing Strategy**:
   - Unit tests within each crate
   - Integration tests across crates
   - End-to-end tests in the main application

4. **Documentation First**:
   - Document APIs before implementation
   - Use doctests to verify examples
   - Keep design documents updated
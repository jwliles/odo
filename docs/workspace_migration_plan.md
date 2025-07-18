# Workspace Migration Plan

This document outlines the plan to migrate Odo to a Cargo workspace structure, with the goal of eventually extracting reusable components into separate repositories.

## Phase 1: Initial Workspace Setup

1. **Create Workspace Structure**

   ```
   Odo/
   ├── Cargo.toml (workspace manifest)
   ├── .cargo/      (workspace configuration)
   │   └── config.toml
   ├── structural-undo/  (undo/redo functionality)
   ├── neo-org-core/     (core document model)
   ├── neo-org-editor/   (editor logic)
   ├── neo-org-terminal/ (terminal UI)
   ├── neo-org-gui/      (GUI implementation)
   └── neo-org/          (main application)
   ```

2. **Update Root Cargo.toml**

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

   [workspace.dependencies]
   # Shared dependencies with fixed versions
   tree-sitter = "0.20.10"
   unicode-segmentation = "1.12.0"
   crossterm = "0.28.1"
   termion = "4.0.5"
   egui = "0.25.0"
   eframe = "0.25.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```

3. **Create Initial Crate Structure**

   For each crate, create a minimal structure with:
   - Cargo.toml with dependencies
   - src/lib.rs with module structure
   - README.md with basic description

## Phase 2: Code Migration

1. **structural-undo Crate**

   Focus on implementing a minimal viable version:
   - Basic Patch and Operation types
   - History Tree structure
   - Simple storage format

2. **neo-org-core Crate**

   Move the core document model:
   - Document, Row, Position structs
   - File I/O functionality
   - Search implementation
   - Base highlighting (without editor specifics)

3. **neo-org-editor Crate**

   Move editor logic:
   - Command processing
   - Mode handling
   - Keybinding infrastructure
   - Status message handling
   - Integration with structural-undo

4. **neo-org-terminal and neo-org-gui Crates**

   Move UI implementations:
   - Terminal rendering and input
   - GUI window and rendering
   - View-specific code

5. **neo-org Application Crate**

   Create application entry point:
   - Main function with CLI parsing
   - Configuration handling
   - Wiring up components

## Phase 3: Feature Completion Within Workspace

1. **structural-undo**

   Implement advanced features:
   - Tree-sitter integration
   - Kill ring
   - Visualization
   - Persistent storage

2. **neo-org-core**

   Enhance core functionality:
   - Org mode structure parsing
   - Tree-sitter integration
   - Improved search

3. **neo-org-editor**

   Add editor capabilities:
   - Complete Vim motions
   - Org-specific commands
   - Multiple document handling

4. **UI Crates**

   Improve UI implementations:
   - Better rendering
   - More widgets and views
   - Performance optimizations

## Phase 4: Extraction Preparation

1. **Dependency Audit**

   For each crate to be extracted:
   - Review dependencies
   - Ensure no workspace-specific code
   - Check for proper API boundaries

2. **Documentation**

   Improve documentation:
   - Add examples
   - Write comprehensive API docs
   - Create usage guides

3. **Testing**

   Enhance test coverage:
   - Unit tests for core functionality
   - Integration tests across crate boundaries
   - Property-based tests for complex components

4. **Version Planning**

   Define versioning strategy:
   - Initial version numbers
   - API stability guarantees
   - Feature roadmaps

## Phase 5: Extraction to Independent Repositories

1. **structural-undo**

   Extract to its own repository:
   - Create new repo
   - Copy code and history
   - Set up CI/CD
   - Publish to crates.io

2. **neo-org-core** and **neo-org-editor**

   Extract as needed:
   - Evaluate if extraction is beneficial
   - Follow same process as structural-undo
   - Update dependencies in main project

3. **Odo Repository Updates**

   Update main project:
   - Switch to using published crates
   - Update workflow for cross-repo development
   - Update contribution guidelines

## Extraction Criteria

A crate is ready for extraction when:

1. **API Stability**: The API is stable and unlikely to undergo major changes
2. **Documentation**: Comprehensive documentation is in place
3. **Test Coverage**: Good test coverage exists
4. **Reusability**: The crate has proven reusable beyond Odo
5. **Demand**: There's actual or potential demand from other projects

## Development Workflow During Transition

1. **Feature Development**

   - Develop features within the workspace
   - Use internal dependencies during development
   - Test across crate boundaries

2. **Bug Fixes**

   - For extracted crates, fix in their repos
   - Create PR in that repo
   - Update dependency in main project

3. **Coordination**

   - Use GitHub projects to track cross-repo work
   - Ensure version compatibility
   - Coordinate releases

## Timeline

- **Phase 1**: 1-2 weeks
- **Phase 2**: 3-4 weeks
- **Phase 3**: 2-3 months
- **Phase 4**: 2-3 weeks per crate
- **Phase 5**: 1-2 weeks per crate

## Specific Tasks for structural-undo Extraction

As the first candidate for extraction, structural-undo should:

1. Have a well-defined API that avoids editor-specific concepts
2. Work with generic content types, not just text
3. Have comprehensive documentation and examples
4. Include examples for different use cases (text editor, recipe editor, etc.)
5. Have a solid test suite covering core functionality

## Post-Extraction Workflow for Recipe Editor Integration

After extracting structural-undo:

1. Create recipe editor project
2. Add structural-undo as a dependency
3. Implement recipe-specific patch handlers
4. Create visualization tailored to recipe operations
5. Share improvements back to the structural-undo project
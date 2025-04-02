# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands
- Build: `cargo build`
- Run: `cargo run [filename]`
- Test: `cargo test`
- Test single test: `cargo test test_name`
- Test specific module: `cargo test --test module_name`
- Clippy check: `cargo clippy -- -D warnings`

## Code Style Guidelines
- Formatting: Follow Rust style with 4-space indentation
- Naming: Use snake_case for variables/functions, CamelCase for types/structs
- Error handling: Use Result<T, E> for operations that can fail
- Clippy: All code adheres to clippy::all, clippy::pedantic (with specific allows)
- Comments: Document public APIs, keep comments concise and meaningful
- Module organization: Follow standard Rust module patterns
- Imports: Group std imports first, then external crates, then internal modules
- Tests: Write unit tests in module's test_super submodule
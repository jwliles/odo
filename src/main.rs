#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod core;
mod editor;
mod ui;

use ui::terminal::TerminalEditor;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // For now we only support terminal UI 
    // In the future, we will parse args to determine the UI to use
    let use_gui = false;
    
    if use_gui {
        // GUI implementation will go here
        println!("GUI mode not yet implemented");
    } else {
        // Terminal UI
        let mut editor = TerminalEditor::default();
        editor.run();
    }
}
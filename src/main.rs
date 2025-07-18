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
use ui::gui::GuiEditor;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if GUI mode is requested with --gui flag
    let use_gui = args.iter().any(|arg| arg == "--gui");
    
    // Get the filename from args (skipping the program name and the --gui flag)
    let mut filename = None;
    for arg in args.iter().skip(1) {
        if arg != "--gui" {
            filename = Some(arg.clone());
            break;
        }
    }
    
    if use_gui {
        // GUI implementation
        let mut editor = GuiEditor::default();
        
        // If a filename was provided, try to open the file
        if let Some(file) = filename {
            match crate::core::Document::open(&file) {
                Ok(document) => {
                    editor = editor.with_document(document);
                },
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                }
            }
        }
        
        if let Err(e) = editor.run() {
            eprintln!("Error running GUI editor: {}", e);
        }
    } else {
        // Terminal UI
        let mut editor = TerminalEditor::default();
        
        // If a filename was provided, the TerminalEditor will handle it
        editor.run();
    }
}
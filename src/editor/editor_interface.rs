use crate::core::{Document, Position, SearchDirection};
use crate::editor::{Mode, StatusMessage};

// This trait defines the interface that any editor UI must implement
pub trait EditorInterface {
    // Core operations
    fn process_keypress(&mut self) -> Result<(), std::io::Error>;
    fn refresh_screen(&mut self) -> Result<(), std::io::Error>;
    
    // Document operations
    fn open_document(&mut self, filename: &str) -> Result<(), std::io::Error>;
    fn save_document(&mut self) -> Result<(), std::io::Error>;
    fn search_document(&mut self) -> Option<String>;
    
    // Editor state
    fn get_document(&self) -> &Document;
    fn get_document_mut(&mut self) -> &mut Document;
    fn get_cursor_position(&self) -> Position;
    fn get_mode(&self) -> Mode;
    
    // UI operations
    fn set_status_message(&mut self, message: StatusMessage);
    fn prompt<F>(&mut self, prompt: &str, callback: F) -> Result<Option<String>, std::io::Error>
    where
        F: FnMut(&mut Self, char, &String);
    
    // Mode operations
    fn enter_insert_mode(&mut self);
    fn enter_command_mode(&mut self);
}
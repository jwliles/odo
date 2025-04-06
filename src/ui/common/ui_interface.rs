use crate::core::{Document, Position, Row};
use crate::editor::StatusMessage;

// This trait defines the interface that all UI implementations must implement
pub trait UserInterface {
    // Drawing operations
    fn draw_rows(&self, document: &Document, offset: &Position) -> Result<(), std::io::Error>;
    fn draw_status_bar(&self, document: &Document, cursor_position: &Position, status: &str) -> Result<(), std::io::Error>;
    fn draw_message_bar(&self, message: &StatusMessage) -> Result<(), std::io::Error>;
    fn clear_screen(&self) -> Result<(), std::io::Error>;
    
    // Input handling
    fn read_key(&self) -> Result<char, std::io::Error>;
    
    // Cursor operations
    fn cursor_position(&self, position: &Position) -> Result<(), std::io::Error>;
    fn cursor_hide(&self) -> Result<(), std::io::Error>;
    fn cursor_show(&self) -> Result<(), std::io::Error>;
    
    // Screen information
    fn size(&self) -> (usize, usize); // width, height
}
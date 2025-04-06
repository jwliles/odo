use crate::core::Position;
use crate::ui::common::ui_interface::UserInterface;
use crate::editor::StatusMessage;
use crate::core::{Document, Row};
use crate::ui::common::theme::Color;
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::time::Duration;
use std::time::Instant;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    
    pub fn size(&self) -> &Size {
        &self.size
    }
    
    // Convert our Color struct to termion's Rgb
    fn to_termion_color(color: &Color) -> color::Rgb {
        color::Rgb(color.r, color.g, color.b)
    }
    
    pub fn set_bg_color(color: &Color) {
        print!("{}", color::Bg(Self::to_termion_color(color)));
    }
    
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    
    pub fn set_fg_color(color: &Color) {
        print!("{}", color::Fg(Self::to_termion_color(color)));
    }
    
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
    
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

impl UserInterface for Terminal {
    fn draw_rows(&self, document: &Document, offset: &Position) -> Result<(), std::io::Error> {
        let height = self.size.height as usize;
        
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = document.row(offset.y.saturating_add(terminal_row)) {
                self.draw_row(row, offset)?;
            } else if document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                println!("~\r");
            }
        }
        
        Ok(())
    }
    
    fn draw_status_bar(&self, document: &Document, cursor_position: &Position, status: &str) -> Result<(), std::io::Error> {
        // Implementation specific to terminal UI
        Ok(())
    }
    
    fn draw_message_bar(&self, message: &StatusMessage) -> Result<(), std::io::Error> {
        Terminal::clear_current_line();
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.size.width as usize);
            print!("{}", text);
        }
        Ok(())
    }
    
    fn clear_screen(&self) -> Result<(), std::io::Error> {
        print!("{}", termion::clear::All);
        Ok(())
    }
    
    fn read_key(&self) -> Result<char, std::io::Error> {
        if let Ok(Key::Char(c)) = Terminal::read_key() {
            Ok(c)
        } else {
            // For simplicity, returning a null character if not a character key
            Ok('\0')
        }
    }
    
    fn cursor_position(&self, position: &Position) -> Result<(), std::io::Error> {
        let Position { x, y } = *position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
        Ok(())
    }
    
    fn cursor_hide(&self) -> Result<(), std::io::Error> {
        print!("{}", termion::cursor::Hide);
        Ok(())
    }
    
    fn cursor_show(&self) -> Result<(), std::io::Error> {
        print!("{}", termion::cursor::Show);
        Ok(())
    }
    
    fn size(&self) -> (usize, usize) {
        (self.size.width as usize, self.size.height as usize)
    }
}

// Terminal-specific implementations
impl Terminal {
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    
    fn draw_welcome_message(&self) -> Result<(), std::io::Error> {
        let version = env!("CARGO_PKG_VERSION");
        let mut welcome_message = format!("Orgonaut editor -- version {}", version);
        let width = self.size.width as usize;
        let len = welcome_message.len();
        
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
        
        Ok(())
    }
    
    fn draw_row(&self, row: &Row, offset: &Position) -> Result<(), std::io::Error> {
        let width = self.size.width as usize;
        let start = offset.x;
        let end = offset.x.saturating_add(width);
        let rendered_row = row.render(start, end);
        println!("{}\r", rendered_row);
        
        Ok(())
    }
    
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
}
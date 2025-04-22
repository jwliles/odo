use crate::core::{Document, Position, SearchDirection};
use crate::editor::{EditorInterface, Mode, StatusMessage, CommandState, Motion, Operator, TextObject};
use crate::ui::terminal::Terminal;
use crate::ui::common::ui_interface::UserInterface;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

pub struct TerminalEditor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
    mode: Mode,
    command_state: CommandState,
    selection_start: Option<Position>,
}

impl Default for TerminalEditor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("NORMAL MODE: i=insert | a=append | Ctrl-F=find | Ctrl-S=save | Ctrl-O=open | Ctrl-Q=quit");

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
            highlighted_word: None,
            mode: Mode::Normal, // Start in Normal Mode (previously called Command)
            command_state: CommandState::new(),
            selection_start: None,
        }
    }
}

impl TerminalEditor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                // Clean up the terminal before exiting
                if let Err(error) = self.terminal.cleanup() {
                    die(error);
                }
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }

    fn draw_row(&self, row: &crate::core::Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("NeoOrg editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_status_bar(&self) {
        use termion::color;
        
        const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
        const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
        
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        
        // Add mode to status bar
        let mode_str = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "VISUAL LINE",
            Mode::Command => "COMMAND",
        };
        
        status = format!(
            "{} - {} lines{} | {}",
            file_name,
            self.document.len(),
            modified_indicator,
            mode_str
        );

        let line_indicator = format!(
            "{} | {}/{}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        #[allow(clippy::integer_arithmetic)]
        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(&crate::ui::common::theme::Color::new(239, 239, 239));
        Terminal::set_fg_color(&crate::ui::common::theme::Color::new(63, 63, 63));
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    
    // Handler for g-prefixed commands (Org-specific navigation)
    fn handle_g_command(&mut self) -> Result<(), std::io::Error> {
        self.status_message = StatusMessage::from(String::from("g"));
        
        // Read the next key after g
        let next_key = Terminal::read_key()?;
        match next_key {
            Key::Char('g') => {
                // Go to beginning of file
                self.cursor_position = Position { x: 0, y: 0 };
                self.command_state.clear();
            }
            Key::Char('h') => {
                // Go to previous heading at same level
                self.status_message = StatusMessage::from(String::from("Previous heading (same level) - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('j') => {
                // Go to next heading
                self.status_message = StatusMessage::from(String::from("Next heading - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('k') => {
                // Go to previous heading
                self.status_message = StatusMessage::from(String::from("Previous heading - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('l') => {
                // Go to next heading at same level
                self.status_message = StatusMessage::from(String::from("Next heading (same level) - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('p') => {
                // Go to parent heading
                self.status_message = StatusMessage::from(String::from("Parent heading - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('c') => {
                // Go to child heading
                self.status_message = StatusMessage::from(String::from("Child heading - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('t') => {
                // Go to next TODO item
                self.status_message = StatusMessage::from(String::from("Next TODO item - Not implemented"));
                self.command_state.clear();
            }
            Key::Char('b') => {
                // Go to next code block
                self.status_message = StatusMessage::from(String::from("Next code block - Not implemented"));
                self.command_state.clear();
            }
            _ => {
                // Unknown g command
                self.command_state.clear();
            }
        }
        
        Ok(())
    }
}

impl EditorInterface for TerminalEditor {
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        
        // Handle global key bindings that work in all modes
        match pressed_key {
            Key::Ctrl('q') => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true;
                return Ok(());
            }
            Key::Ctrl('s') => {
                self.save_document()?;
                return Ok(());
            }
            _ => ()
        }
        
        // Handle mode-specific keybindings
        match self.mode {
            Mode::Normal => {
                // First check if a count digit is being entered
                if let Key::Char(c) = pressed_key {
                    if c.is_ascii_digit() && (c != '0' || self.command_state.has_count()) {
                        self.command_state.parse_count(c);
                        return Ok(());
                    }
                }
                
                match pressed_key {
                    Key::Char('i') => self.enter_insert_mode(),
                    Key::Char('I') => {
                        // Move to first non-blank character on line and enter insert mode
                        self.move_cursor(Key::Home);
                        self.enter_insert_mode();
                    }
                    Key::Char('a') => {
                        // Move cursor right then enter insert mode (append)
                        if let Some(row) = self.document.row(self.cursor_position.y) {
                            if !row.is_empty() && self.cursor_position.x < row.len() {
                                self.move_cursor(Key::Right);
                            }
                        }
                        self.enter_insert_mode();
                    }
                    Key::Char('A') => {
                        // Move to end of line and enter insert mode
                        self.move_cursor(Key::End);
                        self.enter_insert_mode();
                    }
                    Key::Char('o') => {
                        // Open line below cursor and enter insert mode
                        self.move_cursor(Key::End);
                        self.document.insert(&self.cursor_position, '\n');
                        self.move_cursor(Key::Down);
                        self.enter_insert_mode();
                    }
                    Key::Char('O') => {
                        // Open line above cursor and enter insert mode
                        self.move_cursor(Key::Home);
                        self.document.insert(&self.cursor_position, '\n');
                        self.move_cursor(Key::Up);
                        self.enter_insert_mode();
                    }
                    Key::Char('x') => {
                        // Delete character under cursor
                        let count = self.command_state.get_count();
                        for _ in 0..count {
                            self.document.delete(&self.cursor_position);
                        }
                        self.command_state.clear();
                    }
                    Key::Char('d') => {
                        // Delete operator
                        if self.command_state.is_operator_pending() && self.command_state.get_operator() == Some('d') {
                            // Delete current line
                            let count = self.command_state.get_count();
                            // Implementation would go here
                            self.status_message = StatusMessage::from(format!("Delete {} lines", count));
                            self.command_state.clear();
                        } else {
                            // Set operator pending state
                            self.command_state.set_operator_pending('d');
                            self.status_message = StatusMessage::from("d".to_string());
                        }
                    }
                    Key::Char('y') => {
                        // Yank operator
                        if self.command_state.is_operator_pending() && self.command_state.get_operator() == Some('y') {
                            // Yank current line
                            let count = self.command_state.get_count();
                            // Implementation would go here
                            self.status_message = StatusMessage::from(format!("Yank {} lines", count));
                            self.command_state.clear();
                        } else {
                            // Set operator pending state
                            self.command_state.set_operator_pending('y');
                            self.status_message = StatusMessage::from("y".to_string());
                        }
                    }
                    Key::Char('g') => {
                        // Handle g-prefixed commands
                        self.handle_g_command()?;
                    }
                    Key::Char('v') => {
                        // Visual mode
                        self.enter_visual_mode();
                    }
                    Key::Char('V') => {
                        // Visual line mode
                        self.enter_visual_line_mode();
                    }
                    Key::Char(':') => {
                        // Command mode for Ex commands
                        self.enter_command_mode();
                    }
                    Key::Char('/') => {
                        // Search forward
                        self.search_document();
                    }
                    // Navigation keys
                    Key::Char('h') | Key::Left => {
                        self.move_cursor(Key::Left);
                        self.command_state.clear();
                    }
                    Key::Char('j') | Key::Down => {
                        let count = self.command_state.get_count();
                        for _ in 0..count {
                            self.move_cursor(Key::Down);
                        }
                        self.command_state.clear();
                    }
                    Key::Char('k') | Key::Up => {
                        let count = self.command_state.get_count();
                        for _ in 0..count {
                            self.move_cursor(Key::Up);
                        }
                        self.command_state.clear();
                    }
                    Key::Char('l') | Key::Right => {
                        self.move_cursor(Key::Right);
                        self.command_state.clear();
                    }
                    Key::Char('w') => {
                        // Move forward one word
                        let count = self.command_state.get_count();
                        // Implementation would go here
                        self.status_message = StatusMessage::from(format!("Move forward {} words", count));
                        self.command_state.clear();
                    }
                    Key::Char('b') => {
                        // Move backward one word
                        let count = self.command_state.get_count();
                        // Implementation would go here
                        self.status_message = StatusMessage::from(format!("Move backward {} words", count));
                        self.command_state.clear();
                    }
                    Key::Char('e') => {
                        // Move to end of word
                        let count = self.command_state.get_count();
                        // Implementation would go here
                        self.status_message = StatusMessage::from(format!("Move to end of {} words", count));
                        self.command_state.clear();
                    }
                    Key::Char('0') => {
                        // Beginning of line
                        self.move_cursor(Key::Home);
                        self.command_state.clear();
                    }
                    Key::Char('$') => {
                        // End of line
                        self.move_cursor(Key::End);
                        self.command_state.clear();
                    }
                    Key::Char('G') => {
                        // Go to line
                        if self.command_state.has_count() {
                            let line = self.command_state.get_count().saturating_sub(1);
                            if line < self.document.len() {
                                self.cursor_position.y = line;
                                self.cursor_position.x = 0;
                            }
                        } else {
                            // Go to end of file
                            self.cursor_position.y = self.document.len().saturating_sub(1);
                            self.cursor_position.x = 0;
                        }
                        self.command_state.clear();
                    }
                    Key::PageUp | Key::PageDown | Key::End | Key::Home => {
                        self.move_cursor(pressed_key);
                        self.command_state.clear();
                    }
                    Key::Ctrl('f') => {
                        self.search_document();
                        self.command_state.clear();
                    }
                    Key::Ctrl('o') => {
                        // Prompt for filename and open document
                        if self.document.is_dirty() {
                            self.status_message = StatusMessage::from(
                                "WARNING! Current file has unsaved changes.".to_string()
                            );
                            let response = self.prompt("Open new file anyway? (y/n): ", |_, _, _| {}).unwrap_or(None);
                            if response.is_none() || response.unwrap().to_lowercase() != "y" {
                                self.status_message = StatusMessage::from("Open aborted.".to_string());
                                return Ok(());
                            }
                        }

                        let filename = self.prompt("Open file: ", |_, _, _| {}).unwrap_or(None);
                        if let Some(filename) = filename {
                            if let Err(e) = self.open_document(&filename) {
                                self.status_message = StatusMessage::from(format!("Error opening file: {}", e));
                            } else {
                                self.status_message = StatusMessage::from(format!("Opened file: {}", filename));
                                self.cursor_position = Position::default();
                                self.offset = Position::default();
                            }
                        }
                        self.command_state.clear();
                    }
                    _ => {
                        // If an operator is pending but got an invalid motion, clear the state
                        if self.command_state.is_operator_pending() {
                            self.command_state.clear();
                        }
                    }
                }
            }
            Mode::Insert => {
                match pressed_key {
                    Key::Esc => self.enter_normal_mode(),
                    Key::Ctrl('o') => {
                        // Force normal mode and then process the open command
                        self.enter_normal_mode();
                        self.process_keypress()?;
                        self.enter_insert_mode();
                    },
                    Key::Char(c) => {
                        self.document.insert(&self.cursor_position, c);
                        self.move_cursor(Key::Right);
                    }
                    Key::Delete => self.document.delete(&self.cursor_position),
                    Key::Backspace => {
                        if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                            self.move_cursor(Key::Left);
                            self.document.delete(&self.cursor_position);
                        }
                    }
                    Key::Up | Key::Down | Key::Left | Key::Right | 
                    Key::PageUp | Key::PageDown | Key::End | Key::Home => {
                        self.move_cursor(pressed_key);
                    }
                    _ => (),
                }
            }
            Mode::Visual => {
                match pressed_key {
                    Key::Esc => self.enter_normal_mode(),
                    Key::Char('v') => self.enter_normal_mode(),
                    Key::Char('V') => self.enter_visual_line_mode(),
                    Key::Up | Key::Down | Key::Left | Key::Right | 
                    Key::PageUp | Key::PageDown | Key::End | Key::Home |
                    Key::Char('h') | Key::Char('j') | Key::Char('k') | Key::Char('l') => {
                        // Map h,j,k,l to arrow keys
                        let key = match pressed_key {
                            Key::Char('h') => Key::Left,
                            Key::Char('j') => Key::Down,
                            Key::Char('k') => Key::Up,
                            Key::Char('l') => Key::Right,
                            _ => pressed_key
                        };
                        self.move_cursor(key);
                    }
                    // Visual mode operators
                    Key::Char('y') => {
                        // Yank selection
                        self.status_message = StatusMessage::from("Yanked selection".to_string());
                        self.enter_normal_mode();
                    }
                    Key::Char('d') => {
                        // Delete selection
                        self.status_message = StatusMessage::from("Deleted selection".to_string());
                        self.enter_normal_mode();
                    }
                    _ => ()
                }
            }
            Mode::VisualLine => {
                match pressed_key {
                    Key::Esc => self.enter_normal_mode(),
                    Key::Char('v') => self.enter_visual_mode(),
                    Key::Char('V') => self.enter_normal_mode(),
                    Key::Up | Key::Down | Key::Char('j') | Key::Char('k') => {
                        // Only allow vertical movement in visual line mode
                        let key = match pressed_key {
                            Key::Char('j') => Key::Down,
                            Key::Char('k') => Key::Up,
                            _ => pressed_key
                        };
                        self.move_cursor(key);
                    }
                    // Visual line mode operators
                    Key::Char('y') => {
                        // Yank lines
                        self.status_message = StatusMessage::from("Yanked lines".to_string());
                        self.enter_normal_mode();
                    }
                    Key::Char('d') => {
                        // Delete lines
                        self.status_message = StatusMessage::from("Deleted lines".to_string());
                        self.enter_normal_mode();
                    }
                    _ => ()
                }
            }
            Mode::Command => {
                // Command-line mode (:) - not fully implemented yet
                match pressed_key {
                    Key::Esc => self.enter_normal_mode(),
                    _ => {
                        self.status_message = StatusMessage::from("Command mode not fully implemented".to_string());
                        self.enter_normal_mode();
                    }
                }
            }
        }
        
        self.scroll();
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }
    
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide()?;
        Terminal::cursor_position(&Position::default())?;
        
        if self.should_quit {
            self.terminal.clear_screen()?;
            println!("Goodbye.\r");
        } else {
            // Clear screen before redrawing
            self.terminal.clear_screen()?;
            
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows()?;
            self.draw_status_bar();
            self.draw_message_bar();
            self.terminal.cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })?;
        }
        self.terminal.cursor_show()?;
        Terminal::flush()
    }
    
    fn open_document(&mut self, filename: &str) -> Result<(), std::io::Error> {
        match Document::open(filename) {
            Ok(doc) => {
                self.document = doc;
                Ok(())
            },
            Err(e) => {
                self.status_message = StatusMessage::from(format!("Error opening file: {}", e));
                Err(e)
            }
        }
    }
    
    fn save_document(&mut self) -> Result<(), std::io::Error> {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return Ok(());
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
            Ok(())
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to save file"))
        }
    }
    
    fn search_document(&mut self) -> Option<String> {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, ↓/j/n=next, ↑/k/p=prev): ",
                |editor, key, query| {
                    let mut moved = false;
                    // Handle key presses for navigation within search
                    let was_empty = query.is_empty();
                    
                    match key {
                        'n' | 'j' | 'd' | 'l' => {
                            direction = SearchDirection::Forward;
                            moved = true;
                        }
                        'p' | 'k' | 'a' | 'h' => {
                            direction = SearchDirection::Backward;
                            moved = true;
                        }
                        _ => {
                            // For other keys, if this is a new character being typed,
                            // we want to search from the current position forward
                            if !was_empty && !query.is_empty() {
                                direction = SearchDirection::Forward;
                                moved = true;
                            }
                        }
                    }
                    // Only search if query is not empty
                    if !query.is_empty() {
                        if let Some(position) =
                            editor
                                .document
                                .find(&query, &editor.cursor_position, direction)
                        {
                            editor.cursor_position = position;
                            editor.scroll();
                        } else if moved {
                            // If we're moving to next/prev but no results found, wrap around
                            let position = if direction == SearchDirection::Forward {
                                // If searching forward and not found, start from beginning
                                Position { x: 0, y: 0 }
                            } else {
                                // If searching backward and not found, start from end
                                let y = editor.document.len().saturating_sub(1);
                                let x = editor.document.row(y).map_or(0, |r| r.len());
                                Position { x, y }
                            };
                            
                            // Try one more search from the wrapped-around position
                            if let Some(new_position) = editor.document.find(&query, &position, direction) {
                                editor.cursor_position = new_position;
                                editor.scroll();
                                editor.status_message = StatusMessage::from("Search wrapped around".to_string());
                            } else {
                                editor.status_message = StatusMessage::from("No matches found".to_string());
                            }
                        } else {
                            editor.status_message = StatusMessage::from("No matches found".to_string());
                        }
                        editor.highlighted_word = Some(query.to_string());
                    }
                },
            )
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        self.highlighted_word = None;
        query
    }
    
    fn get_document(&self) -> &Document {
        &self.document
    }
    
    fn get_document_mut(&mut self) -> &mut Document {
        &mut self.document
    }
    
    fn get_cursor_position(&self) -> Position {
        self.cursor_position.clone()
    }
    
    fn get_mode(&self) -> Mode {
        self.mode
    }
    
    fn set_status_message(&mut self, message: StatusMessage) {
        self.status_message = message;
    }
    
    fn prompt<F>(&mut self, prompt: &str, mut callback: F) -> Result<Option<String>, std::io::Error>
    where
        F: FnMut(&mut Self, char, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Backspace => result.truncate(result.len().saturating_sub(1)),
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            
            // Call the callback with appropriate character based on key
            match key {
                Key::Char(c) => callback(self, c, &result),
                Key::Up => callback(self, 'k', &result),     // Map Up to 'k'
                Key::Down => callback(self, 'j', &result),   // Map Down to 'j'
                Key::Left => callback(self, 'h', &result),   // Map Left to 'h'
                Key::Right => callback(self, 'l', &result),  // Map Right to 'l'
                _ => (),
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
    
    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.command_state.clear();
        self.selection_start = None;
        self.status_message = StatusMessage::from("-- INSERT MODE --".to_string());
    }
    
    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.command_state.clear();
        self.selection_start = None;
        self.status_message = StatusMessage::from("-- NORMAL MODE --".to_string());
    }
    
    fn enter_visual_mode(&mut self) {
        self.mode = Mode::Visual;
        self.selection_start = Some(self.cursor_position);
        self.command_state.clear();
        self.status_message = StatusMessage::from("-- VISUAL MODE --".to_string());
    }
    
    fn enter_visual_line_mode(&mut self) {
        self.mode = Mode::VisualLine;
        self.selection_start = Some(Position { 
            x: 0, 
            y: self.cursor_position.y 
        });
        self.command_state.clear();
        self.status_message = StatusMessage::from("-- VISUAL LINE MODE --".to_string());
    }
    
    fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command_state.clear();
        self.status_message = StatusMessage::from("-- COMMAND MODE --".to_string());
    }
}

fn die(e: std::io::Error) {
    // This is a utility function to handle fatal errors
    // We create a temporary Terminal instance just to clean up properly
    if let Ok(term) = Terminal::default() {
        let _ = term.cleanup();
    }
    
    // Print error to stderr (will be visible after cleanup)
    eprintln!("Error: {}", e);
    std::process::exit(1);
}
use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mode {
    Command,
    Insert,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
    mode: Mode,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("COMMAND MODE: i=insert | a=append | Ctrl-F=find | Ctrl-S=save | Ctrl-Q=quit");

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
            mode: Mode::Command, // Start in Command Mode
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }
    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        }
                        Key::Left | Key::Up => direction = SearchDirection::Backward,
                        _ => direction = SearchDirection::Forward,
                    }
                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                    editor.highlighted_word = Some(query.to_string());
                },
            )
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        self.highlighted_word = None;
    }
    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.status_message = StatusMessage::from("-- INSERT MODE --".to_string());
    }
    
    fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.status_message = StatusMessage::from("-- COMMAND MODE --".to_string());
    }
    
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        
        match self.mode {
            Mode::Command => {
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
                        self.should_quit = true
                    }
                    Key::Ctrl('s') => self.save(),
                    Key::Ctrl('f') => self.search(),
                    Key::Char('i') => self.enter_insert_mode(),
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
                    Key::Char('I') => {
                        // Move to start of line and enter insert mode
                        self.move_cursor(Key::Home);
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
                    Key::Char('x') => self.document.delete(&self.cursor_position),
                    Key::Up
                    | Key::Down
                    | Key::Left
                    | Key::Right
                    | Key::PageUp
                    | Key::PageDown
                    | Key::End
                    | Key::Home => self.move_cursor(pressed_key),
                    _ => (),
                }
            }
            Mode::Insert => {
                match pressed_key {
                    Key::Esc => self.enter_command_mode(),
                    Key::Ctrl('q') => {
                        if self.quit_times > 0 && self.document.is_dirty() {
                            self.status_message = StatusMessage::from(format!(
                                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                                self.quit_times
                            ));
                            self.quit_times -= 1;
                            return Ok(());
                        }
                        self.should_quit = true
                    }
                    Key::Ctrl('s') => self.save(),
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
                    Key::Up
                    | Key::Down
                    | Key::Left
                    | Key::Right
                    | Key::PageUp
                    | Key::PageDown
                    | Key::End
                    | Key::Home => self.move_cursor(pressed_key),
                    _ => (),
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
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Orgonaut editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    #[allow(clippy::integer_division, clippy::integer_arithmetic)]
    fn draw_rows(&self) {
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
    }
    fn draw_status_bar(&self) {
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
            Mode::Command => "COMMAND",
            Mode::Insert => "INSERT",
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
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
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
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
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
            callback(self, key, &result);
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    
    // Mock structs for testing
    // MockDocument is used to simulate Document functionality for tests
    #[allow(clippy::partial_pub_fields)]
    #[derive(Clone)]
    struct MockDocument {
        pub file_name: Option<String>,
        pub rows: Vec<String>,
        is_dirty: bool,
    }
    
    impl MockDocument {
        fn default() -> Self {
            Self {
                file_name: None,
                rows: Vec::new(),
                is_dirty: false,
            }
        }
        
        fn len(&self) -> usize {
            self.rows.len()
        }
        
        fn is_empty(&self) -> bool {
            self.rows.is_empty()
        }
        
        fn is_dirty(&self) -> bool {
            self.is_dirty
        }
        
        fn row(&self, idx: usize) -> Option<&String> {
            self.rows.get(idx)
        }
        
        fn insert(&mut self, _pos: &Position, _c: char) {
            self.is_dirty = true;
        }
        
        fn delete(&mut self, _pos: &Position) {
            self.is_dirty = true;
        }
        
        fn save(&mut self) -> io::Result<()> {
            self.is_dirty = false;
            Ok(())
        }
        
        fn file_type(&self) -> String {
            "txt".to_string()
        }
        
        fn highlight(&mut self, _word: &Option<String>, _until: Option<usize>) {}
        
        fn find(&self, query: &str, _pos: &Position, _dir: SearchDirection) -> Option<Position> {
            if query.is_empty() {
                None
            } else {
                Some(Position { x: 0, y: 0 })
            }
        }
    }
    
    struct MockTerminal {
        width: u16,
        height: u16,
    }
    
    impl MockTerminal {
        fn default() -> io::Result<Self> {
            Ok(Self {
                width: 80,
                height: 24,
            })
        }
        
        fn size(&self) -> Size {
            Size {
                width: self.width,
                height: self.height,
            }
        }
    }
    
    struct Size {
        width: u16,
        height: u16,
    }
    
    // Create a testable version of Editor
    struct TestableEditor {
        should_quit: bool,
        cursor_position: Position,
        offset: Position,
        document: MockDocument,
        quit_times: u8,
        highlighted_word: Option<String>,
        terminal: MockTerminal,
        mode: Mode,
    }
    
    impl TestableEditor {
        fn new() -> Self {
            Self {
                should_quit: false,
                cursor_position: Position::default(),
                offset: Position::default(),
                document: MockDocument::default(),
                quit_times: QUIT_TIMES,
                highlighted_word: None,
                terminal: MockTerminal::default().unwrap(),
                mode: Mode::Command, // Start in Command Mode
            }
        }
        
        fn with_document(mut self, doc: MockDocument) -> Self {
            self.document = doc;
            self
        }
        
        fn enter_insert_mode(&mut self) {
            self.mode = Mode::Insert;
        }
        
        fn enter_command_mode(&mut self) {
            self.mode = Mode::Command;
        }
        
        fn process_key(&mut self, key: Key) {
            match self.mode {
                Mode::Command => {
                    match key {
                        Key::Ctrl('q') => {
                            if self.quit_times > 0 && self.document.is_dirty() {
                                self.quit_times -= 1;
                                return;
                            }
                            self.should_quit = true;
                        }
                        Key::Ctrl('s') => {
                            let _ = self.document.save();
                        }
                        Key::Char('i') => self.enter_insert_mode(),
                        Key::Char('a') => {
                            if let Some(row) = self.document.row(self.cursor_position.y) {
                                if !row.is_empty() && self.cursor_position.x < row.len() {
                                    self.move_cursor(Key::Right);
                                }
                            }
                            self.enter_insert_mode();
                        }
                        Key::Char('A') => {
                            self.move_cursor(Key::End);
                            self.enter_insert_mode();
                        }
                        Key::Char('I') => {
                            self.move_cursor(Key::Home);
                            self.enter_insert_mode();
                        }
                        Key::Char('x') => {
                            self.document.delete(&self.cursor_position);
                        }
                        Key::Up | Key::Down | Key::Left | Key::Right | Key::PageUp 
                        | Key::PageDown | Key::End | Key::Home => {
                            self.move_cursor(key);
                        }
                        _ => (),
                    }
                }
                Mode::Insert => {
                    match key {
                        Key::Esc => self.enter_command_mode(),
                        Key::Ctrl('q') => {
                            if self.quit_times > 0 && self.document.is_dirty() {
                                self.quit_times -= 1;
                                return;
                            }
                            self.should_quit = true;
                        }
                        Key::Ctrl('s') => {
                            let _ = self.document.save();
                        }
                        Key::Char(c) => {
                            self.document.insert(&self.cursor_position, c);
                            self.move_cursor(Key::Right);
                        }
                        Key::Delete => {
                            self.document.delete(&self.cursor_position);
                        }
                        Key::Backspace => {
                            if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                                self.move_cursor(Key::Left);
                                self.document.delete(&self.cursor_position);
                            }
                        }
                        Key::Up | Key::Down | Key::Left | Key::Right | Key::PageUp 
                        | Key::PageDown | Key::End | Key::Home => {
                            self.move_cursor(key);
                        }
                        _ => (),
                    }
                }
            }
            
            if self.quit_times < QUIT_TIMES {
                self.quit_times = QUIT_TIMES;
            }
        }
        
        fn move_cursor(&mut self, key: Key) {
            let Position { mut y, mut x } = self.cursor_position;
            let height = self.document.len();
            let mut width = 0;
            
            if let Some(row) = self.document.row(y) {
                width = row.len();
            }
            
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
                    y = if y > self.terminal.size().height as usize {
                        y.saturating_sub(self.terminal.size().height as usize)
                    } else {
                        0
                    }
                }
                Key::PageDown => {
                    y = if y.saturating_add(self.terminal.size().height as usize) < height {
                        y.saturating_add(self.terminal.size().height as usize)
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
    }
    
    #[test]
    fn test_position_default() {
        let position = Position::default();
        assert_eq!(position.x, 0);
        assert_eq!(position.y, 0);
    }
    
    #[test]
    fn test_status_message() {
        let message = "Test message";
        let status = StatusMessage::from(message.to_string());
        assert_eq!(status.text, message);
    }
    
    #[test]
    fn test_move_cursor_up() {
        let mut editor = TestableEditor::new();
        editor.cursor_position = Position { x: 5, y: 5 };
        
        // Need to provide document with enough rows
        editor.document.rows = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
            "line5".to_string(),
            "line6".to_string(),
        ];
        
        editor.move_cursor(Key::Up);
        assert_eq!(editor.cursor_position.y, 4);
        assert_eq!(editor.cursor_position.x, 5);
    }
    
    #[test]
    fn test_move_cursor_down() {
        let doc = MockDocument {
            file_name: None,
            rows: vec!["line1".to_string(), "line2".to_string(), "line3".to_string()],
            is_dirty: false,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        editor.cursor_position = Position { x: 0, y: 0 };
        
        editor.move_cursor(Key::Down);
        assert_eq!(editor.cursor_position.y, 1);
        assert_eq!(editor.cursor_position.x, 0);
    }
    
    #[test]
    fn test_move_cursor_left() {
        let mut editor = TestableEditor::new();
        editor.cursor_position = Position { x: 5, y: 5 };
        
        // Need to provide document with enough rows
        editor.document.rows = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
            "line5".to_string(),
            "line with enough length".to_string(),
        ];
        
        editor.move_cursor(Key::Left);
        assert_eq!(editor.cursor_position.y, 5);
        assert_eq!(editor.cursor_position.x, 4);
    }
    
    #[test]
    fn test_move_cursor_right() {
        let doc = MockDocument {
            file_name: None,
            rows: vec!["line1".to_string()],
            is_dirty: false,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        editor.cursor_position = Position { x: 0, y: 0 };
        
        editor.move_cursor(Key::Right);
        assert_eq!(editor.cursor_position.y, 0);
        assert_eq!(editor.cursor_position.x, 1);
    }
    
    #[test]
    fn test_move_cursor_to_beginning_of_line() {
        let mut editor = TestableEditor::new();
        editor.cursor_position = Position { x: 5, y: 5 };
        
        editor.move_cursor(Key::Home);
        assert_eq!(editor.cursor_position.y, 5);
        assert_eq!(editor.cursor_position.x, 0);
    }
    
    #[test]
    fn test_move_cursor_to_end_of_line() {
        let doc = MockDocument {
            file_name: None,
            rows: vec!["line1".to_string()],
            is_dirty: false,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        editor.cursor_position = Position { x: 0, y: 0 };
        
        editor.move_cursor(Key::End);
        assert_eq!(editor.cursor_position.y, 0);
        assert_eq!(editor.cursor_position.x, 5); // "line1" length
    }
    
    #[test]
    fn test_process_key_quit() {
        let mut editor = TestableEditor::new();
        
        editor.process_key(Key::Ctrl('q'));
        assert!(editor.should_quit);
    }
    
    #[test]
    fn test_process_key_quit_with_dirty_document() {
        let doc = MockDocument {
            file_name: None,
            rows: vec![],
            is_dirty: true,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        
        // First attempt doesn't quit, decrements counter
        editor.process_key(Key::Ctrl('q'));
        assert!(!editor.should_quit);
        assert_eq!(editor.quit_times, QUIT_TIMES - 1);
        
        // After QUIT_TIMES attempts, it should quit
        for _ in 0..QUIT_TIMES {
            editor.process_key(Key::Ctrl('q'));
        }
        assert!(editor.should_quit);
    }
    
    #[test]
    fn test_process_key_save() {
        let doc = MockDocument {
            file_name: Some("test.txt".to_string()),
            rows: vec!["line1".to_string()],
            is_dirty: true,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        
        editor.process_key(Key::Ctrl('s'));
        assert!(!editor.document.is_dirty());
    }
    
    #[test]
    fn test_process_key_character_insertion() {
        let mut editor = TestableEditor::new();
        // Need to provide document with at least one row
        editor.document.rows = vec!["line1".to_string()];
        
        // First enter insert mode
        editor.enter_insert_mode();
        
        editor.process_key(Key::Char('a'));
        assert!(editor.document.is_dirty());
        assert_eq!(editor.cursor_position.x, 1); // Cursor moved right
    }
    
    #[test]
    fn test_process_key_delete() {
        let doc = MockDocument {
            file_name: None,
            rows: vec!["line1".to_string()],
            is_dirty: false,
        };
        
        let mut editor = TestableEditor::new().with_document(doc);
        
        // Characters can be deleted in both Command and Insert mode
        // In this test we'll use Command mode with 'x'
        editor.process_key(Key::Char('x'));
        assert!(editor.document.is_dirty());
    }
    
    #[test]
    fn test_process_key_backspace() {
        let mut editor = TestableEditor::new();
        editor.cursor_position = Position { x: 5, y: 0 };
        // Need to provide document with at least one row of sufficient length
        editor.document.rows = vec!["line with enough characters".to_string()];
        
        // First enter insert mode since backspace only works in insert mode
        editor.enter_insert_mode();
        
        editor.process_key(Key::Backspace);
        assert!(editor.document.is_dirty());
        assert_eq!(editor.cursor_position.x, 4); // Cursor moved left
    }
    
    #[test]
    fn test_move_cursor_page_up() {
        let mut editor = TestableEditor::new();
        // Create a document with many rows
        editor.document.rows = (0..50).map(|i| format!("line {}", i)).collect();
        editor.cursor_position = Position { x: 0, y: 30 };
        
        editor.move_cursor(Key::PageUp);
        // Should move up by terminal height
        assert_eq!(editor.cursor_position.y, 30 - editor.terminal.size().height as usize);
    }
    
    #[test]
    fn test_move_cursor_page_down() {
        let mut editor = TestableEditor::new();
        // Create a document with many rows
        editor.document.rows = (0..50).map(|i| format!("line {}", i)).collect();
        editor.cursor_position = Position { x: 0, y: 10 };
        
        editor.move_cursor(Key::PageDown);
        // Should move down by terminal height
        assert_eq!(editor.cursor_position.y, 10 + editor.terminal.size().height as usize);
    }
    
    #[test]
    fn test_cursor_position_at_document_end() {
        let mut editor = TestableEditor::new();
        // Create a document with 3 rows
        editor.document.rows = vec![
            "line 1".to_string(),
            "line 2".to_string(),
            "line 3".to_string(),
        ];
        
        // Try to move cursor beyond document end
        editor.cursor_position = Position { x: 0, y: 2 };
        editor.move_cursor(Key::Down);
        
        // Y should be capped at document length
        assert_eq!(editor.cursor_position.y, 3);
    }
    
    #[test]
    fn test_cursor_position_at_line_end() {
        let mut editor = TestableEditor::new();
        // Create a document with a row that has 6 characters
        editor.document.rows = vec!["line 1".to_string()];
        
        // Move cursor beyond line end
        editor.cursor_position = Position { x: 6, y: 0 };
        
        // Try to move cursor right at end of line
        editor.move_cursor(Key::Right);
        
        // X should be 0 and Y should be 1 (next line)
        assert_eq!(editor.cursor_position.x, 0);
        assert_eq!(editor.cursor_position.y, 1);
    }
    
    #[test]
    fn test_wrapping_cursor_when_moving_left_at_line_start() {
        let mut editor = TestableEditor::new();
        // Create a document with multiple rows
        editor.document.rows = vec![
            "line 1".to_string(),
            "line 2".to_string(),
        ];
        
        // Position cursor at start of second line
        editor.cursor_position = Position { x: 0, y: 1 };
        
        // Try to move cursor left at start of line
        editor.move_cursor(Key::Left);
        
        // X should be at end of previous line and Y should be decremented
        assert_eq!(editor.cursor_position.y, 0);
        assert_eq!(editor.cursor_position.x, 6); // Length of "line 1"
    }
    
    #[test]
    fn test_mode_initialization() {
        let editor = TestableEditor::new();
        // Editor should start in Command mode
        assert_eq!(editor.mode, Mode::Command);
    }
    
    #[test]
    fn test_enter_insert_mode() {
        let mut editor = TestableEditor::new();
        editor.enter_insert_mode();
        assert_eq!(editor.mode, Mode::Insert);
    }
    
    #[test]
    fn test_enter_command_mode() {
        let mut editor = TestableEditor::new();
        editor.enter_insert_mode(); // First switch to insert mode
        editor.enter_command_mode();
        assert_eq!(editor.mode, Mode::Command);
    }
    
    #[test]
    fn test_process_key_i_enters_insert_mode() {
        let mut editor = TestableEditor::new();
        editor.process_key(Key::Char('i'));
        assert_eq!(editor.mode, Mode::Insert);
    }
    
    #[test]
    fn test_process_key_esc_returns_to_command_mode() {
        let mut editor = TestableEditor::new();
        editor.enter_insert_mode();
        editor.process_key(Key::Esc);
        assert_eq!(editor.mode, Mode::Command);
    }
    
    #[test]
    fn test_insert_only_works_in_insert_mode() {
        // Setup test editor in command mode with a document
        let mut editor = TestableEditor::new();
        editor.document.rows = vec!["test".to_string()];
        let initial_dirty_state = editor.document.is_dirty();
        
        // Try to insert a character while in command mode
        editor.process_key(Key::Char('a'));
        
        // Should change to insert mode but not insert the 'a' character
        assert_eq!(editor.mode, Mode::Insert);
        assert_eq!(editor.document.is_dirty(), initial_dirty_state);
        
        // Now in insert mode, insert a character
        editor.process_key(Key::Char('b'));
        
        // The document should now be dirty (modified)
        assert!(editor.document.is_dirty());
    }
}

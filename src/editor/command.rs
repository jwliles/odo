use crate::core::Position;

/// Represents the current command state in Normal/Visual modes
pub struct CommandState {
    /// Buffer for multi-key commands
    buffer: Vec<char>,
    /// Flag for operator-pending state
    operator_pending: bool,
    /// Current operator (if any)
    current_operator: Option<char>,
    /// Count prefix for commands
    count: Option<usize>,
    /// Starting position for a command (used for selections)
    start_position: Option<Position>,
}

impl CommandState {
    /// Create a new empty command state
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            operator_pending: false,
            current_operator: None,
            count: None,
            start_position: None,
        }
    }

    /// Add a character to the command buffer
    pub fn push(&mut self, c: char) {
        self.buffer.push(c);
    }

    /// Clear the current command state
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.operator_pending = false;
        self.current_operator = None;
        self.count = None;
        self.start_position = None;
    }

    /// Check if the command state is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty() && self.current_operator.is_none() && self.count.is_none()
    }

    /// Check if we're waiting for a motion after an operator
    pub fn is_operator_pending(&self) -> bool {
        self.operator_pending
    }

    /// Set the operator pending state
    pub fn set_operator_pending(&mut self, op: char) {
        self.operator_pending = true;
        self.current_operator = Some(op);
    }

    /// Get the current operator
    pub fn get_operator(&self) -> Option<char> {
        self.current_operator
    }

    /// Parse a count from digit characters
    pub fn parse_count(&mut self, c: char) -> bool {
        if c.is_ascii_digit() {
            // Don't allow counts that start with 0 unless it's the only digit
            if c == '0' && self.count.is_none() {
                self.count = Some(0);
                return true;
            }
            
            let digit = c.to_digit(10).unwrap() as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return true;
        }
        false
    }

    /// Get the current count (default to 1 if not set)
    pub fn get_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Check if a count was explicitly provided
    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    /// Set the starting position for a command
    pub fn set_start_position(&mut self, position: Position) {
        self.start_position = Some(position);
    }

    /// Get the starting position for a command
    pub fn get_start_position(&self) -> Option<Position> {
        self.start_position
    }

    /// Get the entire command as a string
    pub fn as_string(&self) -> String {
        let mut result = String::new();
        
        // Add count if present
        if let Some(count) = self.count {
            result.push_str(&count.to_string());
        }
        
        // Add operator if present
        if let Some(op) = self.current_operator {
            result.push(op);
        }
        
        // Add buffer contents
        for c in &self.buffer {
            result.push(*c);
        }
        
        result
    }
}

/// Text objects for commands like 'daw', 'ciw', etc.
pub enum TextObject {
    Word,           // w
    InnerWord,      // iw
    AroundWord,     // aw
    InnerParagraph, // ip
    AroundParagraph,// ap
    InnerBlock,     // ib
    AroundBlock,    // ab
    InnerQuote,     // i"
    AroundQuote,    // a"
    // Org-specific text objects
    Heading,        // ih
    AroundHeading,  // ah (including content)
    ListItem,       // il
    AroundListItem, // al
    CodeBlock,      // ic
    AroundCodeBlock,// ac
}

/// Operator type for commands like 'd', 'c', 'y'
pub enum Operator {
    Delete, // d
    Change, // c
    Yank,   // y
    Indent, // >
    Outdent,// <
    Format, // =
}

/// Motion type for commands like 'w', 'b', 'j'
pub enum Motion {
    // Character movements
    Left,            // h
    Down,            // j
    Up,              // k
    Right,           // l
    
    // Word movements
    WordForward,     // w
    WordBackward,    // b
    WordEnd,         // e
    
    // Line movements
    LineStart,       // 0
    LineFirstChar,   // ^
    LineEnd,         // $
    
    // Document movements
    FileStart,       // gg
    FileEnd,         // G
    
    // Paragraph movements
    ParagraphForward, // }
    ParagraphBackward,// {
    
    // Search movements
    FindChar,        // f{char}
    TillChar,        // t{char}
    FindCharBack,    // F{char}
    TillCharBack,    // T{char}
    
    // Org-specific movements
    HeadingNext,     // gj
    HeadingPrev,     // gk
    HeadingSameLevelNext, // gh
    HeadingSameLevelPrev, // gl
    ParentHeading,   // gp
    ChildHeading,    // gc
    ListItemNext,    // gn
    ListItemPrev,    // gN
    CodeBlockNext,   // gb
    CodeBlockPrev,   // gB
    TodoItemNext,    // gt
    TodoItemPrev,    // gT
}
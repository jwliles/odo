#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mode {
    Normal,    // Standard Vim navigation mode (renamed from Command)
    Insert,    // Text insertion mode
    Visual,    // Character-based visual selection
    VisualLine, // Line-based visual selection
    Command,   // Command-line mode (for : commands)
}
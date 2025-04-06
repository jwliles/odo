// RGB color definition
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// Theme contains all the color definitions for the editor
pub struct Theme {
    pub status_fg: Color,
    pub status_bg: Color,
    pub default_fg: Color,
    pub default_bg: Color,
    pub error_fg: Color,
    pub comment_fg: Color,
    pub keyword_fg: Color,
    pub headline_fg: Color,
    pub highlight_match_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            status_fg: Color::new(63, 63, 63),    // Dark gray
            status_bg: Color::new(239, 239, 239), // Light gray
            default_fg: Color::new(255, 255, 255), // White
            default_bg: Color::new(0, 0, 0),      // Black
            error_fg: Color::new(255, 0, 0),      // Red
            comment_fg: Color::new(110, 110, 110), // Gray
            keyword_fg: Color::new(0, 135, 255),  // Blue
            headline_fg: Color::new(0, 175, 0),   // Green
            highlight_match_bg: Color::new(45, 45, 45), // Dark gray
        }
    }
}
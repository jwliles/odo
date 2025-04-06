use termion::color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
    // Org mode specific types
    OrgHeadline,
    OrgTodo,
    OrgDone,
    OrgTag,
    OrgList,
    OrgBold,
    OrgItalic,
    OrgUnderline,
    OrgLink,
    OrgCodeBlock,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment | Type::MultilineComment => color::Rgb(133, 153, 0),
            Type::PrimaryKeywords => color::Rgb(181, 137, 0),
            Type::SecondaryKeywords => color::Rgb(42, 161, 152),
            Type::OrgHeadline => color::Rgb(0, 175, 135), // Teal
            Type::OrgTodo => color::Rgb(255, 100, 100), // Red
            Type::OrgDone => color::Rgb(100, 255, 100), // Green
            Type::OrgTag => color::Rgb(150, 150, 255), // Blue
            Type::OrgList => color::Rgb(255, 200, 50), // Orange
            Type::OrgBold => color::Rgb(255, 255, 255), // White
            Type::OrgItalic => color::Rgb(200, 200, 200), // Light gray
            Type::OrgUnderline => color::Rgb(255, 255, 200), // Light yellow
            Type::OrgLink => color::Rgb(100, 100, 255), // Blue
            Type::OrgCodeBlock => color::Rgb(150, 255, 150), // Light green
            _ => color::Rgb(255, 255, 255),
        }
    }
}
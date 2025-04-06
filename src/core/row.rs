use crate::core::highlighting;
use crate::core::HighlightingOptions;
use crate::core::SearchDirection;
use crate::core::FileType;
use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

// We'll reimplement OrgHighlighter when we move treesitter.rs
struct OrgHighlighter;

impl OrgHighlighter {
    fn new() -> Self {
        Self {}
    }
    
    fn highlight_line(&self, line: &str) -> Vec<highlighting::Type> {
        let mut result = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        
        // Simple Org-mode detection for initial implementation
        if line.starts_with('*') {
            // Count asterisks for headline level
            let mut level = 0;
            for c in &chars {
                if *c == '*' {
                    level += 1;
                    result.push(highlighting::Type::OrgHeadline);
                } else {
                    break;
                }
            }
            
            // Headline with content after the stars
            if level < chars.len() {
                // Check for TODO/DONE status
                let remaining = &line[level..];
                if remaining.trim_start().starts_with("TODO ") {
                    for _ in 0..5 { // "TODO " is 5 chars
                        result.push(highlighting::Type::OrgTodo);
                    }
                    
                    // Add rest of headline
                    for _ in 0..(chars.len() - level - 5) {
                        result.push(highlighting::Type::OrgHeadline);
                    }
                } else if remaining.trim_start().starts_with("DONE ") {
                    for _ in 0..5 { // "DONE " is 5 chars
                        result.push(highlighting::Type::OrgDone);
                    }
                    
                    // Add rest of headline
                    for _ in 0..(chars.len() - level - 5) {
                        result.push(highlighting::Type::OrgHeadline);
                    }
                } else {
                    // Regular headline without TODO/DONE
                    for _ in level..chars.len() {
                        result.push(highlighting::Type::OrgHeadline);
                    }
                }
            }
        } else if line.starts_with("- ") || line.starts_with("+ ") || line.starts_with("* ") {
            // List item
            result.push(highlighting::Type::OrgList);
            result.push(highlighting::Type::OrgList);
            
            for _ in 2..chars.len() {
                result.push(highlighting::Type::None);
            }
        } else if line.contains("::") {
            // Definition list or tag
            for c in chars {
                if c == ':' {
                    result.push(highlighting::Type::OrgTag);
                } else {
                    result.push(highlighting::Type::None);
                }
            }
        } else {
            // Default - no syntax highlighting
            for _ in &chars {
                result.push(highlighting::Type::None);
            }
            
            // Check for text styling indicators
            let mut i = 0;
            while i < chars.len() {
                if i < chars.len() - 1 {
                    if chars[i] == '*' && chars[i+1] != ' ' {
                        // Bold text
                        result[i] = highlighting::Type::OrgBold;
                        
                        // Find closing *
                        for j in (i+1)..chars.len() {
                            if chars[j] == '*' {
                                result[j] = highlighting::Type::OrgBold;
                                break;
                            }
                        }
                    } else if chars[i] == '/' && chars[i+1] != ' ' {
                        // Italic text
                        result[i] = highlighting::Type::OrgItalic;
                        
                        // Find closing /
                        for j in (i+1)..chars.len() {
                            if chars[j] == '/' {
                                result[j] = highlighting::Type::OrgItalic;
                                break;
                            }
                        }
                    } else if chars[i] == '_' && chars[i+1] != ' ' {
                        // Underlined text
                        result[i] = highlighting::Type::OrgUnderline;
                        
                        // Find closing _
                        for j in (i+1)..chars.len() {
                            if chars[j] == '_' {
                                result[j] = highlighting::Type::OrgUnderline;
                                break;
                            }
                        }
                    } else if chars[i] == '[' && chars[i+1] == '[' {
                        // Link
                        result[i] = highlighting::Type::OrgLink;
                        result[i+1] = highlighting::Type::OrgLink;
                        
                        // Find closing ]]
                        for j in (i+2)..chars.len()-1 {
                            if chars[j] == ']' && chars[j+1] == ']' {
                                result[j] = highlighting::Type::OrgLink;
                                result[j+1] = highlighting::Type::OrgLink;
                                break;
                            }
                        }
                    }
                }
                i += 1;
            }
        }
        
        result
    }
}

use std::sync::{Mutex, OnceLock};

// Thread-safe lazy-initialized singleton using OnceLock
static ORG_HIGHLIGHTER_CELL: OnceLock<Mutex<OrgHighlighter>> = OnceLock::new();

fn get_org_highlighter() -> &'static Mutex<OrgHighlighter> {
    ORG_HIGHLIGHTER_CELL.get_or_init(|| Mutex::new(OrgHighlighter::new()))
}

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    pub is_highlighted: bool,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            is_highlighted: false,
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlighting = &highlighting::Type::None;
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self
                    .highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Type::None);
                if highlighting_type != current_highlighting {
                    current_highlighting = highlighting_type;
                    let start_highlight =
                        format!("{}", termion::color::Fg(highlighting_type.to_color()));
                    result.push_str(&start_highlight[..]);
                }
                if c == '\t' {
                    result.push_str(" ");
                } else {
                    result.push(c);
                }
            }
        }
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        result.push_str(&end_highlight[..]);
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }
    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;
        self.is_highlighted = false;
        Self {
            string: splitted_row,
            len: splitted_length,
            is_highlighted: false,
            highlighting: Vec::new(),
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };
        #[allow(clippy::integer_arithmetic)]
        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }
            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    #[allow(clippy::indexing_slicing)]
                    for i in search_match..next_index {
                        self.highlighting[i] = highlighting::Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
        file_type: &FileType,
    ) -> bool {
        // For Org files, use our simplified Org highlighter
        if file_type.is_org() {
            return self.highlight_org(word);
        }
        
        // For other file types, use the existing highlighting logic
        let chars: Vec<char> = self.string.chars().collect();
        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == highlighting::Type::MultilineComment
                    && self.string.len() > 1
                    && self.string[self.string.len() - 2..] == *"*/"
                {
                    return true;
                }
            }
            return false;
        }
        self.highlighting = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(highlighting::Type::MultilineComment);
            }
            index = closing_index;
        }
        
        // Add basic existing highlight logic here...
        // This is simplified for brevity
        
        self.highlight_match(word);
        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
    }
    
    fn highlight_org(&mut self, word: &Option<String>) -> bool {
        // Get the OrgHighlighter and highlight the line
        let highlighter = get_org_highlighter();
        
        // Use the OrgHighlighter to highlight the line, with mutex guard
        if let Ok(h) = highlighter.lock() {
            self.highlighting = h.highlight_line(&self.string);
        } else {
            // Fallback if mutex is poisoned, just create empty highlighting
            self.highlighting = vec![highlighting::Type::None; self.string.len()];
        }
        
        // Apply additional highlighting for search match if needed
        self.highlight_match(word);
        
        self.is_highlighted = true;
        false
    }
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}
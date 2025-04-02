use unicode_segmentation::UnicodeSegmentation;
use crate::highlighting::Type as HighlightType;

// A simple Org-mode highlighter until we fully integrate Tree-sitter
pub struct OrgHighlighter;

impl OrgHighlighter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightType> {
        let graphemes: Vec<&str> = line.graphemes(true).collect();
        let mut result = vec![HighlightType::None; graphemes.len()];
        
        // Check for headline markers (* at beginning of line)
        if line.starts_with('*') {
            let mut i = 0;
            // Highlight heading stars
            while i < graphemes.len() && graphemes[i] == "*" {
                result[i] = HighlightType::PrimaryKeywords;
                i += 1;
            }
            
            // Skip whitespace
            while i < graphemes.len() && graphemes[i].trim().is_empty() {
                i += 1;
            }
            
            // Check for TODO keywords
            if i < graphemes.len() {
                let rest = &line[i..];
                if rest.starts_with("TODO ") {
                    for _ in 0..4 { // "TODO" length
                        if i < graphemes.len() {
                            result[i] = HighlightType::SecondaryKeywords;
                            i += 1;
                        }
                    }
                } else if rest.starts_with("DONE ") {
                    for _ in 0..4 { // "DONE" length
                        if i < graphemes.len() {
                            result[i] = HighlightType::SecondaryKeywords;
                            i += 1;
                        }
                    }
                }
            }
            
            // If tags present at end of line [:tag:], highlight them
            let _tag_pattern = ":[a-zA-Z0-9_]+:";
            if let Some(tag_index) = line.find(':') {
                if tag_index > 0 && line[tag_index..].contains(':') {
                    // Simple check for tag-like pattern
                    for (j, _) in graphemes.iter().enumerate().skip(tag_index) {
                        if j < graphemes.len() {
                            result[j] = HighlightType::Character;
                        }
                    }
                }
            }
        }
        
        // Check for Org markup
        self.highlight_markup(line, &mut result, &graphemes);
        
        // Check for list items
        if line.trim().starts_with("- ") {
            let list_prefix_len = line.find("- ").unwrap() + 2;
            for i in 0..list_prefix_len {
                if i < graphemes.len() {
                    result[i] = HighlightType::Number;
                }
            }
        }
        
        // Check for checkboxes [X] or [ ]
        if line.contains("[ ]") || line.contains("[X]") {
            let checkbox_pattern = if line.contains("[ ]") { "[ ]" } else { "[X]" };
            if let Some(checkbox_index) = line.find(checkbox_pattern) {
                for i in checkbox_index..(checkbox_index + 3) {
                    if i < graphemes.len() {
                        result[i] = HighlightType::SecondaryKeywords;
                    }
                }
            }
        }
        
        // Check for Org special lines (#+KEYWORD:)
        if line.starts_with("#+") {
            let keyword_end = line.find(':').unwrap_or(line.len());
            for i in 0..keyword_end + 1 {
                if i < graphemes.len() {
                    result[i] = HighlightType::SecondaryKeywords;
                }
            }
        }
        
        // Check for source blocks
        if line.starts_with("#+BEGIN_SRC") || line.starts_with("#+END_SRC") {
            for i in 0..line.len() {
                if i < graphemes.len() {
                    result[i] = HighlightType::String;
                }
            }
        }
        
        // Check for comment lines
        if line.starts_with("#") && !line.starts_with("#+") {
            for i in 0..line.len() {
                if i < graphemes.len() {
                    result[i] = HighlightType::Comment;
                }
            }
        }
        
        result
    }
    
    fn highlight_markup(&self, line: &str, result: &mut Vec<HighlightType>, graphemes: &[&str]) {
        // Bold: *bold*
        self.highlight_pattern_pairs(line, result, graphemes, "*", "*", HighlightType::Number);
        
        // Italic: /italic/
        self.highlight_pattern_pairs(line, result, graphemes, "/", "/", HighlightType::String);
        
        // Underline: _underline_
        self.highlight_pattern_pairs(line, result, graphemes, "_", "_", HighlightType::Character);
        
        // Strikethrough: +strikethrough+
        self.highlight_pattern_pairs(line, result, graphemes, "+", "+", HighlightType::MultilineComment);
        
        // Code: ~code~
        self.highlight_pattern_pairs(line, result, graphemes, "~", "~", HighlightType::String);
        
        // Verbatim: =verbatim=
        self.highlight_pattern_pairs(line, result, graphemes, "=", "=", HighlightType::String);
        
        // Links: [[link][description]]
        if line.contains("[[") && line.contains("]]") {
            let mut i = 0;
            while i < line.len() {
                if i + 1 < line.len() && &line[i..i+2] == "[[" {
                    let link_start = i;
                    if let Some(link_end) = line[i..].find("]]") {
                        let link_end = i + link_end + 2;
                        for j in link_start..link_end {
                            if j < graphemes.len() {
                                result[j] = HighlightType::Character;
                            }
                        }
                        i = link_end;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }
    
    fn highlight_pattern_pairs(
        &self, 
        line: &str, 
        result: &mut Vec<HighlightType>, 
        graphemes: &[&str],
        start_pattern: &str,
        end_pattern: &str,
        highlight_type: HighlightType
    ) {
        let mut i = 0;
        while i < line.len() {
            if i + start_pattern.len() <= line.len() && &line[i..i+start_pattern.len()] == start_pattern {
                let marker_start = i;
                i += start_pattern.len();
                
                if let Some(remaining) = line.get(i..) {
                    if let Some(end_idx) = remaining.find(end_pattern) {
                        let end_idx = i + end_idx;
                        
                        // Highlight everything including the markers
                        for j in marker_start..end_idx + end_pattern.len() {
                            if j < graphemes.len() {
                                result[j] = highlight_type;
                            }
                        }
                        
                        i = end_idx + end_pattern.len();
                        continue;
                    }
                }
            }
            i += 1;
        }
    }
}
use tree_sitter::{Language, Parser, Query, QueryCursor};
use std::collections::HashMap;
use std::sync::Once;
use std::str::FromStr;

use crate::highlighting::Type as HighlightType;

extern "C" {
    fn tree_sitter_org() -> Language;
}

// Ensure we only initialize the language once
static INIT_ORG: Once = Once::new();
static mut ORG_LANGUAGE: Option<Language> = None;

// Org-mode node types that we're interested in highlighting
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OrgNodeType {
    Headline,
    TodoKeyword,
    Tag,
    ListItem,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    Code,
    Verbatim,
    Timestamp,
    Link,
    Comment,
    SrcBlock,
    ExampleBlock,
    QuoteBlock,
    Priority,
    SpecialBlock,
    Unknown,
}

impl FromStr for OrgNodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "headline" => Ok(OrgNodeType::Headline),
            "todo_keyword" => Ok(OrgNodeType::TodoKeyword),
            "tag" => Ok(OrgNodeType::Tag),
            "list_item" => Ok(OrgNodeType::ListItem),
            "bold" => Ok(OrgNodeType::Bold),
            "italic" => Ok(OrgNodeType::Italic),
            "underline" => Ok(OrgNodeType::Underline),
            "strikethrough" => Ok(OrgNodeType::StrikeThrough),
            "code" => Ok(OrgNodeType::Code),
            "verbatim" => Ok(OrgNodeType::Verbatim),
            "timestamp" => Ok(OrgNodeType::Timestamp),
            "link" => Ok(OrgNodeType::Link),
            "comment" => Ok(OrgNodeType::Comment),
            "src_block" => Ok(OrgNodeType::SrcBlock),
            "example_block" => Ok(OrgNodeType::ExampleBlock),
            "quote_block" => Ok(OrgNodeType::QuoteBlock),
            "priority" => Ok(OrgNodeType::Priority),
            "special_block" => Ok(OrgNodeType::SpecialBlock),
            _ => Ok(OrgNodeType::Unknown),
        }
    }
}

impl OrgNodeType {
    // Map Org-mode node types to our editor's highlighting types
    pub fn to_highlight_type(&self) -> HighlightType {
        match self {
            OrgNodeType::Headline => HighlightType::PrimaryKeywords,
            OrgNodeType::TodoKeyword => HighlightType::SecondaryKeywords,
            OrgNodeType::Tag => HighlightType::Character,
            OrgNodeType::ListItem => HighlightType::Number,
            OrgNodeType::Bold => HighlightType::Number,
            OrgNodeType::Italic => HighlightType::String,
            OrgNodeType::Underline => HighlightType::Character,
            OrgNodeType::StrikeThrough => HighlightType::MultilineComment,
            OrgNodeType::Code => HighlightType::String,
            OrgNodeType::Verbatim => HighlightType::String,
            OrgNodeType::Timestamp => HighlightType::Number,
            OrgNodeType::Link => HighlightType::Character,
            OrgNodeType::Comment => HighlightType::Comment,
            OrgNodeType::SrcBlock => HighlightType::String,
            OrgNodeType::ExampleBlock => HighlightType::String,
            OrgNodeType::QuoteBlock => HighlightType::MultilineComment,
            OrgNodeType::Priority => HighlightType::SecondaryKeywords,
            OrgNodeType::SpecialBlock => HighlightType::Character,
            OrgNodeType::Unknown => HighlightType::None,
        }
    }
}

pub struct TreeSitterHighlighter {
    parser: Parser,
    org_query: Query,
}

impl TreeSitterHighlighter {
    pub fn new() -> Self {
        // Initialize the Org language
        INIT_ORG.call_once(|| {
            unsafe {
                ORG_LANGUAGE = Some(tree_sitter_org());
            }
        });

        let language = unsafe { ORG_LANGUAGE.unwrap() };
        
        // Create a parser
        let mut parser = Parser::new();
        parser.set_language(language).expect("Error setting language");

        // Query to capture nodes we want to highlight
        let query_source = r#"
            (headline) @headline
            (todo_keyword) @todo_keyword
            (tag) @tag
            (list_item) @list_item
            (bold) @bold
            (italic) @italic
            (underline) @underline
            (strikethrough) @strikethrough
            (code) @code
            (verbatim) @verbatim
            (timestamp) @timestamp
            (link) @link
            (comment) @comment
            (src_block) @src_block
            (example_block) @example_block
            (quote_block) @quote_block
            (priority) @priority
            (special_block) @special_block
        "#;

        let org_query = Query::new(language, query_source).expect("Error creating query");

        Self {
            parser,
            org_query,
        }
    }

    pub fn highlight_text(&mut self, text: &str) -> HashMap<usize, HighlightType> {
        let mut result = HashMap::new();
        
        // Parse the text
        let tree = self.parser.parse(text, None).expect("Failed to parse text");
        let root_node = tree.root_node();
        
        // Use query to find nodes to highlight
        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&self.org_query, root_node, text.as_bytes());
        
        for match_ in matches {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = self.org_query.capture_names()[capture.index as usize];
                let node_type = OrgNodeType::from_str(capture_name).unwrap_or(OrgNodeType::Unknown);
                let highlight_type = node_type.to_highlight_type();
                
                // Get the byte range of the node
                let start_byte = node.start_byte();
                let end_byte = node.end_byte();
                
                // Convert byte positions to character indices and store highlight type
                // Note: This is a simplified approach. In a real implementation,
                // you'd need to convert byte positions to grapheme cluster indices.
                let mut byte_idx = start_byte;
                while byte_idx < end_byte {
                    if let Some(c) = text.as_bytes().get(byte_idx) {
                        // Skip continuation bytes in UTF-8
                        if (*c & 0xC0) != 0x80 {
                            result.insert(byte_idx, highlight_type);
                        }
                        byte_idx += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        
        result
    }
}
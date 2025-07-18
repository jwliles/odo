# Org-mode-optimized Motion Implementation Plan

## Overview

This document outlines the implementation plan for adding Org-mode-optimized motions to Odo. Taking inspiration from Vim's efficient approach to keyboard navigation, we'll create intuitive motions specifically designed for Org-mode's unique structure, enabling users to navigate and edit Org documents with minimal keystrokes.

## Design Principles

1. **Vim-inspired simplicity**: Create simple, memorable keybindings inspired by Vim's philosophy of efficiency.
2. **Org-mode first**: Design motions that are fundamentally optimized for Org-mode structure.
3. **Intuitive mappings**: Create logical, easy-to-learn keybindings that feel natural for Org-mode editing.
4. **Minimal keystrokes**: Prioritize commands that accomplish common tasks with the fewest keystrokes.
5. **Maintain essential Vim keybindings**: Preserve widely-used, intuitive Vim motions to maintain familiarity.
6. **Progressive implementation**: Start with the most useful motions, then expand the system over time.

## Preserved Standard Motions

We'll maintain compatibility with these essential Vim motions:

| Key       | Action                          | Rationale                                      |
|-----------|----------------------------------|------------------------------------------------|
| `/`       | Search forward                   | Universal, intuitive search operation          |
| `*`       | Search for word under cursor     | Efficient for referencing the current word     |
| `gg`      | Go to first line                 | Quick navigation to document start             |
| `G`       | Go to last line                  | Quick navigation to document end               |
| `Ctrl+G`  | Show cursor position/file info   | Useful status information                      |
| `0`, `^`  | Go to start of line              | Common line navigation                         |
| `$`       | Go to end of line                | Common line navigation                         |
| `w`, `b`, `e` | Word navigation              | Fundamental text navigation                     |

These standard motions will be preserved while we implement our Org-specific extensions.

## Implementation Approach

### 1. Standard Motion Integration

Ensure compatibility with standard Vim motions by implementing them in the command mode keypress handler:

```rust
impl TerminalEditor {
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // ...
        match self.mode {
            Mode::Command => {
                match key {
                    // Standard Vim motions
                    Key::Char('/') => {
                        // Initiate forward search
                        self.search_document();
                    },
                    Key::Char('*') => {
                        // Search for word under cursor
                        if let Some(word) = self.get_word_under_cursor() {
                            self.search_specific_term(&word);
                        }
                    },
                    Key::Char('g') => {
                        // Check for 'gg' to go to document start
                        if let Key::Char('g') = Terminal::read_key()? {
                            self.cursor_position = Position { x: 0, y: 0 };
                            self.scroll();
                        } else {
                            // This will be expanded for our Org-specific 'g' commands
                        }
                    },
                    Key::Char('G') => {
                        // Go to document end
                        let y = self.document.len().saturating_sub(1);
                        self.cursor_position = Position { 
                            x: 0,
                            y
                        };
                        self.scroll();
                    },
                    Key::Ctrl('g') => {
                        // Show cursor position information
                        let total_lines = self.document.len();
                        let current_line = self.cursor_position.y + 1;
                        let percentage = if total_lines > 0 {
                            (current_line * 100) / total_lines
                        } else {
                            100
                        };
                        self.status_message = StatusMessage::from(
                            format!("Line {} of {} ({}%)", current_line, total_lines, percentage)
                        );
                    },
                    
                    // Line navigation
                    Key::Char('0') => {
                        // Go to start of line
                        self.cursor_position.x = 0;
                    },
                    Key::Char('$') => {
                        // Go to end of line
                        if let Some(row) = self.document.row(self.cursor_position.y) {
                            self.cursor_position.x = row.len();
                        }
                    },
                    
                    // Our Org-specific motions will expand from here
                    // ...
                }
            }
        }
    }
}
```

### 2. Document Structure Parser

Before implementing motions, we need to enhance our document parser to recognize Org-mode structural elements:

```rust
// Extend the existing Document struct
impl Document {
    // Find the next/previous heading at the same level
    pub fn next_heading_same_level(&self, current_line: usize) -> Option<usize> { ... }
    pub fn prev_heading_same_level(&self, current_line: usize) -> Option<usize> { ... }
    
    // Find parent/child headings
    pub fn parent_heading(&self, current_line: usize) -> Option<usize> { ... }
    pub fn next_child_heading(&self, current_line: usize) -> Option<usize> { ... }
    
    // Find next/previous list item
    pub fn next_list_item(&self, current_line: usize) -> Option<usize> { ... }
    pub fn prev_list_item(&self, current_line: usize) -> Option<usize> { ... }
    
    // Find TODO items
    pub fn next_todo_item(&self, current_line: usize) -> Option<usize> { ... }
    pub fn prev_todo_item(&self, current_line: usize) -> Option<usize> { ... }
    
    // Find code blocks
    pub fn next_code_block(&self, current_line: usize) -> Option<usize> { ... }
    pub fn prev_code_block(&self, current_line: usize) -> Option<usize> { ... }
    pub fn find_code_block_end(&self, current_line: usize) -> Option<usize> { ... }
    
    // Find matching begin/end pairs
    pub fn find_matching_pair(&self, current_line: usize) -> Option<usize> { ... }
}
```

### 3. Basic Heading Navigation

Implement intuitive navigation between headings with the `g` prefix:

```rust
impl TerminalEditor {
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // ...
        match self.mode {
            Mode::Command => {
                match key {
                    // ... existing key mappings
                    
                    // Org navigation with 'g' prefix
                    Key::Char('g') => {
                        // Check for second command character
                        match Terminal::read_key()? {
                            Key::Char('g') => {
                                // Go to beginning of document (standard Vim motion)
                                self.cursor_position = Position { x: 0, y: 0 };
                                self.scroll();
                            },
                            Key::Char('h') => {
                                // Jump to previous heading at same level (g+h = go heading previous)
                                if let Some(line) = self.document.prev_heading_same_level(self.cursor_position.y) {
                                    self.cursor_position.y = line;
                                    self.cursor_position.x = 0;
                                    self.scroll();
                                }
                            },
                            Key::Char('j') => {
                                // Jump to next heading at same level (g+j = go heading next)
                                if let Some(line) = self.document.next_heading_same_level(self.cursor_position.y) {
                                    self.cursor_position.y = line;
                                    self.cursor_position.x = 0;
                                    self.scroll();
                                }
                            },
                            Key::Char('p') => {
                                // Jump to parent heading (g+p = go parent)
                                if let Some(line) = self.document.parent_heading(self.cursor_position.y) {
                                    self.cursor_position.y = line;
                                    self.cursor_position.x = 0;
                                    self.scroll();
                                }
                            },
                            Key::Char('c') => {
                                // Jump to child heading (g+c = go child)
                                if let Some(line) = self.document.next_child_heading(self.cursor_position.y) {
                                    self.cursor_position.y = line;
                                    self.cursor_position.x = 0;
                                    self.scroll();
                                }
                            },
                            // ... other 'g' prefixed commands will go here
                            _ => {}
                        }
                    },
                    
                    // ... more key mappings
                }
            }
            // ... other modes
        }
    }
}
```

### 4. List Navigation

Add list navigation to the `g` prefix commands:

```rust
// Add to the 'g' prefix handler:
Key::Char('g') => {
    match Terminal::read_key()? {
        // ... existing 'g' commands
        
        Key::Char('l') => {
            // Jump to previous list item (g+l = go list previous)
            if let Some(line) = self.document.prev_list_item(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        Key::Char('n') => {
            // Jump to next list item (g+n = go next list)
            if let Some(line) = self.document.next_list_item(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        // ... other 'g' commands
    }
}
```

### 5. TODO Item Navigation

Add TODO navigation to the existing command system:

```rust
// Add to the 'g' prefix handler:
Key::Char('g') => {
    match Terminal::read_key()? {
        // ... existing 'g' commands
        
        Key::Char('t') => {
            // Jump to next TODO item (g+t = go to next todo)
            if let Some(line) = self.document.next_todo_item(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        Key::Char('T') => {
            // Jump to previous TODO item (g+T = go to previous todo)
            if let Some(line) = self.document.prev_todo_item(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        // ... other 'g' commands
    }
}
```

### 6. Code Block Navigation

Add code block navigation to our intuitive command system:

```rust
// Add to the 'g' prefix handler:
Key::Char('g') => {
    match Terminal::read_key()? {
        // ... existing 'g' commands
        
        Key::Char('b') => {
            // Jump to beginning of next code block (g+b = go to block beginning)
            if let Some(line) = self.document.next_code_block(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        Key::Char('e') => {
            // Jump to end of current code block (g+e = go to block end)
            if let Some(line) = self.document.find_code_block_end(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        // ... other 'g' commands
    }
}
```

### 7. Matching Block Navigation

Create a simple motion for navigating between matching blocks:

```rust
// Add to the 'g' prefix handler:
Key::Char('g') => {
    match Terminal::read_key()? {
        // ... existing 'g' commands
        
        Key::Char('m') => {
            // Jump to matching begin/end pair (g+m = go to match)
            if let Some(line) = self.document.find_matching_pair(self.cursor_position.y) {
                self.cursor_position.y = line;
                self.cursor_position.x = 0;
                self.scroll();
            }
        },
        // ... other 'g' commands
    }
}
```

### 8. Folding Functionality

Implement simple, intuitive folding with the `z` key:

```rust
// Add to the keypress handler:
Key::Char('z') => {
    // Simple toggle fold at cursor position
    if self.is_heading_at_cursor() {
        self.toggle_fold_at_cursor();
    }
}

// Add these methods to TerminalEditor:
impl TerminalEditor {
    fn is_heading_at_cursor(&self) -> bool {
        // Check if cursor is on a heading
        if let Some(row) = self.document.row(self.cursor_position.y) {
            // Check if row starts with one or more asterisks followed by a space
            let content = row.render(0, row.len());
            let trimmed = content.trim_start();
            trimmed.starts_with('*') && trimmed.chars().skip(1).take_while(|&c| c == '*').count() > 0
                && trimmed.chars().nth(trimmed.chars().take_while(|&c| c == '*').count()) == Some(' ')
        } else {
            false
        }
    }
    
    fn toggle_fold_at_cursor(&mut self) {
        // If this heading is already folded, unfold it
        // If this heading is not folded, fold it
        // Implementation will depend on how folding is tracked in the Document
    }
}
```

## Structural Text Operations

Implementing direct operations on structural elements for efficient editing:

```rust
// Add to the keypress handler:
Key::Char('d') => {
    // Check for second character for structural deletion
    match Terminal::read_key()? {
        Key::Char('h') => {
            // Delete heading and its content (d+h = delete heading)
            self.delete_heading_at_cursor();
        },
        Key::Char('l') => {
            // Delete list item (d+l = delete list)
            self.delete_list_item_at_cursor();
        },
        Key::Char('c') => {
            // Delete code block (d+c = delete code block)
            self.delete_code_block_at_cursor();
        },
        // ... existing 'd' commands for characters, lines, etc.
        _ => {}
    }
}

// Similar pattern for other operations:
Key::Char('y') => {
    // Check for second character for structural copying
    match Terminal::read_key()? {
        Key::Char('h') => {
            // Yank (copy) heading (y+h = yank heading)
            self.yank_heading_at_cursor();
        },
        // ... other structural elements
        _ => {}
    }
}

Key::Char('c') => {
    // Check for second character for structural changing
    match Terminal::read_key()? {
        Key::Char('h') => {
            // Change heading text (c+h = change heading)
            self.change_heading_at_cursor();
        },
        // ... other structural elements
        _ => {}
    }
}
```

## Structure Editing Commands

Implement intuitive commands for manipulating document structure:

```rust
// Add to the keypress handler:
Key::Char('+') => {
    // Check for second character
    match Terminal::read_key()? {
        Key::Char('h') => {
            // Promote heading (decrease level: *** → **)
            self.promote_heading_at_cursor();
        },
        Key::Char('l') => {
            // Promote list item
            self.promote_list_item_at_cursor();
        },
        _ => {}
    }
}

Key::Char('-') => {
    // Check for second character
    match Terminal::read_key()? {
        Key::Char('h') => {
            // Demote heading (increase level: ** → ***)
            self.demote_heading_at_cursor();
        },
        Key::Char('l') => {
            // Demote list item
            self.demote_list_item_at_cursor();
        },
        _ => {}
    }
}

Key::Char('t') => {
    // Check for second character
    if Terminal::read_key()? == Key::Char('t') {
        // Double 't' to cycle TODO state
        self.cycle_todo_state_at_cursor();
    }
}
```

## Testing Approach

1. Create a comprehensive suite of test Org files with different structural elements:
   - Headings at various levels
   - Different list types (ordered, unordered, nested)
   - TODO items with different states
   - Code blocks with various languages
   - Special blocks (quotes, examples)
   - Tags in different contexts

2. Develop unit tests for each structural navigation and operation:
   - Test that each motion correctly identifies the target
   - Verify that operations preserve document integrity
   - Ensure that cursor positioning is accurate after operations

3. Create integration tests for common editing workflows:
   - Reorganizing document structure
   - Managing TODO items
   - Manipulating lists
   - Folding and unfolding sections

4. Conduct usability testing with Org-mode users:
   - Test intuitiveness of keybindings
   - Gather feedback on command ergonomics
   - Adjust mappings based on real-world usage

## Phased Implementation

### Phase 0: Standard Motion Support
- Ensure all standard Vim motions work correctly:
  - Basic movement (h, j, k, l)
  - Search (/, ?, *)
  - Line navigation (0, ^, $)
  - Document navigation (gg, G)
  - Word navigation (w, b, e)
  - Status information (Ctrl+G)

### Phase 1: Document Structure Analysis
- Enhance document parser to recognize Org structural elements
- Develop robust heading level detection
- Implement list item identification (all types)
- Add TODO state recognition
- Add code block detection

### Phase 2: Navigation Framework
- Implement the 'g' prefix command system
- Add basic heading navigation (gh, gj, gp, gc)
- Add list navigation (gl, gn)
- Add structure-aware scrolling

### Phase 3: Enhanced Navigation
- Implement TODO item navigation (gt, gT)
- Add code block navigation (gb, ge)
- Add block matching navigation (gm)
- Implement tag navigation (g#)

### Phase 4: Structural Operations
- Implement heading operations (dh, yh, ch)
- Add list operations (dl, yl, cl)
- Add code block operations (dc, yc, cc)
- Implement basic structure selection

### Phase 5: Folding
- Implement simple folding with z toggle
- Add fold level management
- Add visual indicators for folded content

### Phase 6: Document Manipulation
- Implement heading promotion/demotion (+h, -h)
- Add list promotion/demotion (+l, -l)
- Implement TODO state cycling (tt)
- Add tag editing functionality (#)
- Implement structure rearrangement (m)

## Conclusion

This implementation plan provides a roadmap for adding Org-mode-optimized motions to Odo. Unlike simply porting Vim commands, these motions are designed specifically for the structure and workflow of Org-mode documents. By following this plan, we can create an editor that combines the efficiency of modal editing with deep understanding of Org-mode's unique characteristics, delivering a powerful, intuitive editing experience tailored specifically for Org documents.
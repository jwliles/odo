// GUI implementation using egui and eframe
use crate::core::{Document, Position};
use crate::editor::StatusMessage;
use crate::ui::common::ui_interface::UserInterface;
use egui::{FontId, TextEdit, FontDefinitions, FontFamily};
use eframe::egui;

pub struct GuiEditor {
    document: Document,
    cursor_position: Position,
    status_message: StatusMessage,
    window_size: (usize, usize),
    font_size: f32,
    font_family: String,
    show_settings: bool,
    available_fonts: Vec<String>,
    text_editor_id: egui::Id,
    font_search: String,
    fonts_loaded: bool,
}

impl Default for GuiEditor {
    fn default() -> Self {
        let mut available_fonts = vec![
            "monospace".to_string(),
            "proportional".to_string(),
        ];
        
        // Add discovered user fonts
        let user_fonts = Self::discover_user_fonts();
        for (font_name, _) in user_fonts {
            available_fonts.push(font_name);
        }
        
        // Add system fonts
        for font_name in [
            "DejaVu Sans Mono",
            "Liberation Mono", 
            "Source Code Pro",
            "Fira Code",
            "Hack",
            "JetBrains Mono"
        ] {
            available_fonts.push(font_name.to_string());
        }
        
        // Sort fonts alphabetically (keep monospace and proportional first)
        let mut sorted_fonts = vec!["monospace".to_string(), "proportional".to_string()];
        let mut other_fonts: Vec<String> = available_fonts.into_iter().skip(2).collect();
        other_fonts.sort();
        sorted_fonts.extend(other_fonts);
        available_fonts = sorted_fonts;
        
        Self {
            document: Document::default(),
            cursor_position: Position { x: 0, y: 0 },
            status_message: StatusMessage::from("HELP: Ctrl-Q = quit | Ctrl-S = save"),
            window_size: (80, 30),
            font_size: 14.0,
            font_family: "monospace".to_string(),
            show_settings: false,
            available_fonts,
            text_editor_id: egui::Id::new("main_text_editor"),
            font_search: String::new(),
            fonts_loaded: false,
        }
    }
}

impl GuiEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_document(mut self, document: Document) -> Self {
        self.document = document;
        self
    }
    

    fn discover_user_fonts() -> Vec<(String, String)> {
        let mut fonts = Vec::new();
        let home_dir = std::env::var("HOME").unwrap_or_default();
        let fonts_dir = format!("{}/.local/share/fonts", home_dir);
        
        if let Ok(entries) = std::fs::read_dir(&fonts_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        // Check for TTF/OTF files in subdirectories
                        let subdir = entry.path();
                        if let Ok(font_files) = std::fs::read_dir(&subdir) {
                            for font_file in font_files.flatten() {
                                let path = font_file.path();
                                if let Some(ext) = path.extension() {
                                    if ext == "ttf" || ext == "otf" {
                                        if let Some(font_name) = path.file_stem() {
                                            fonts.push((
                                                font_name.to_string_lossy().to_string(),
                                                path.to_string_lossy().to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(ext) = entry.path().extension() {
                        if ext == "ttf" || ext == "otf" {
                            if let Some(font_name) = entry.path().file_stem() {
                                fonts.push((
                                    font_name.to_string_lossy().to_string(),
                                    entry.path().to_string_lossy().to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        fonts
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        
        // Add user fonts dynamically
        #[cfg(target_os = "linux")]
        {
            let user_fonts = Self::discover_user_fonts();
            for (font_name, path) in user_fonts {
                if let Ok(font_data) = std::fs::read(&path) {
                    fonts.font_data.insert(font_name.clone(), egui::FontData::from_owned(font_data));
                    fonts.families.insert(
                        FontFamily::Name(font_name.clone().into()),
                        vec![font_name.clone()],
                    );
                }
            }
            
            // System fonts as fallback
            for (font_name, path) in [
                ("DejaVu Sans Mono", "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"),
                ("Liberation Mono", "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf"),
                ("Source Code Pro", "/usr/share/fonts/TTF/SourceCodePro-Regular.ttf"),
                ("Fira Code", "/usr/share/fonts/truetype/firacode/FiraCode-Regular.ttf"),
                ("Hack", "/usr/share/fonts/TTF/Hack-Regular.ttf"),
                ("JetBrains Mono", "/usr/share/fonts/truetype/jetbrains-mono/JetBrainsMono-Regular.ttf"),
            ] {
                if let Ok(font_data) = std::fs::read(path) {
                    fonts.font_data.insert(font_name.to_owned(), egui::FontData::from_owned(font_data));
                    fonts.families.insert(
                        FontFamily::Name(font_name.into()),
                        vec![font_name.to_owned()],
                    );
                }
            }
        }
        
        ctx.set_fonts(fonts);
    }

    pub fn run(self) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(egui::vec2(1200.0, 800.0))
                .with_min_inner_size(egui::vec2(800.0, 600.0)),
            ..Default::default()
        };
        eframe::run_native(
            "Odo Editor",
            options,
            Box::new(|cc| {
                Self::setup_fonts(&cc.egui_ctx);
                Box::new(self)
            }),
        )
    }
}

impl eframe::App for GuiEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load fonts once
        if !self.fonts_loaded {
            Self::setup_fonts(ctx);
            self.fonts_loaded = true;
        }
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Org files", &["org"])
                            .add_filter("All files", &["*"])
                            .pick_file() 
                        {
                            match Document::open(&path.to_string_lossy()) {
                                Ok(doc) => {
                                    self.document = doc;
                                    self.status_message = StatusMessage::from(&format!("File opened: {}", path.to_string_lossy()));
                                },
                                Err(e) => {
                                    self.status_message = StatusMessage::from(&format!("Error opening file: {}", e));
                                }
                            }
                        }
                    }
                    if ui.button("Save").clicked() {
                        if let Some(_) = &self.document.file_name {
                            match self.document.save() {
                                Ok(_) => {
                                    self.status_message = StatusMessage::from("File saved successfully");
                                },
                                Err(e) => {
                                    self.status_message = StatusMessage::from(&format!("Error saving file: {}", e));
                                }
                            }
                        } else {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Org files", &["org"])
                                .add_filter("All files", &["*"])
                                .save_file() 
                            {
                                self.document.file_name = Some(path.to_string_lossy().to_string());
                                match self.document.save() {
                                    Ok(_) => {
                                        self.status_message = StatusMessage::from("File saved successfully");
                                    },
                                    Err(e) => {
                                        self.status_message = StatusMessage::from(&format!("Error saving file: {}", e));
                                    }
                                }
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    ui.add_enabled_ui(false, |ui| {
                        let _ = ui.button("Cut");
                        let _ = ui.button("Copy");
                        let _ = ui.button("Paste");
                    });
                });
                ui.menu_button("Settings", |ui| {
                    if ui.button("Font Settings").clicked() {
                        self.show_settings = true;
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.status_message = StatusMessage::from("Odo - A versatile text editor with first-class Org support");
                    }
                });
            });
        });

        // Main editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the document content
            let mut text = String::new();
            if self.document.rows.is_empty() {
                text = "No content loaded. Try typing here or use File > Open to load a document.".to_string();
            } else {
                for row in &self.document.rows {
                    text.push_str(row.as_string());
                    text.push('\n');
                }
            }
            
            let font_id = if self.font_family == "monospace" {
                FontId::monospace(self.font_size)
            } else if self.font_family == "proportional" {
                FontId::proportional(self.font_size)
            } else {
                // For custom fonts, we need to ensure they're loaded
                let font_family = egui::FontFamily::Name(self.font_family.clone().into());
                FontId::new(self.font_size, font_family)
            };
            
            // Simple text editor that fills available space with persistent ID
            let text_edit = TextEdit::multiline(&mut text)
                .font(font_id)
                .desired_width(f32::INFINITY)
                .desired_rows(0)
                .code_editor()
                .hint_text("Start typing or use File > Open to load a document...")
                .id(self.text_editor_id);
            
            let response = ui.add_sized(ui.available_size(), text_edit);
            
            // Keep focus on the text editor to preserve selection
            if response.has_focus() {
                ui.memory_mut(|mem| mem.set_focus_lock_filter(self.text_editor_id, egui::EventFilter::default()));
            }
            
            // Handle text changes
            if response.changed() {
                // Update the document with the new text
                self.document.rows.clear();
                for line in text.lines() {
                    self.document.insert_row(line);
                }
                
                // If the document is empty and had no lines, add an empty line
                if self.document.rows.is_empty() {
                    self.document.insert_row("");
                }
                
                // Simplified cursor position tracking
                let y = text.lines().count().saturating_sub(1);
                let x = if y < self.document.rows.len() {
                    self.document.rows[y].len()
                } else {
                    0
                };
                self.cursor_position = Position { x, y };
            }
        });
        
        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(filename) = &self.document.file_name {
                    ui.label(filename);
                } else {
                    ui.label("[No File]");
                }
                
                let cursor_text = format!("{},{}", self.cursor_position.y + 1, self.cursor_position.x + 1);
                ui.label(cursor_text);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    let status_message = self.status_message.text.clone();
                    ui.label(status_message);
                });
            });
        });
        
        // Settings window
        if self.show_settings {
            let settings_response = egui::Window::new("Font Settings")
                .collapsible(false)
                .resizable(true)
                .default_width(400.0)
                .default_height(300.0)
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(12.0, 16.0);
                    ui.spacing_mut().button_padding = egui::vec2(16.0, 8.0);
                    
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label("Font Family:");
                        ui.add_space(12.0);
                        
                        // Searchable font selection
                        ui.vertical(|ui| {
                            // Search box
                            ui.add(egui::TextEdit::singleline(&mut self.font_search)
                                .hint_text("Type to search fonts...")
                                .desired_width(200.0));
                            
                            ui.add_space(4.0);
                            
                            // Filtered font list
                            let filtered_fonts: Vec<String> = if self.font_search.is_empty() {
                                self.available_fonts.clone()
                            } else {
                                self.available_fonts.iter()
                                    .filter(|font| font.to_lowercase().contains(&self.font_search.to_lowercase()))
                                    .cloned()
                                    .collect()
                            };
                            
                            egui::ComboBox::from_id_source("font_family")
                                .selected_text(&self.font_family)
                                .width(200.0)
                                .show_ui(ui, |ui| {
                                    for font in &filtered_fonts {
                                        let display_name = if font == "monospace" {
                                            "Monospace (Default)".to_string()
                                        } else if font == "proportional" {
                                            "Proportional".to_string()
                                        } else {
                                            font.clone()
                                        };
                                        if ui.selectable_value(&mut self.font_family, font.clone(), display_name).clicked() {
                                            self.font_search.clear(); // Clear search when font is selected
                                        }
                                    }
                                });
                        });
                    });
                    
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label("Font Size:");
                        ui.add_space(12.0);
                        ui.add(egui::Slider::new(&mut self.font_size, 8.0..=32.0)
                            .suffix(" pt")
                            .text("Size"));
                    });
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        if ui.button("Reset to Defaults").clicked() {
                            self.font_family = "monospace".to_string();
                            self.font_size = 14.0;
                            self.font_search.clear();
                        }
                    });
                });
            
            // Close settings when clicking outside the window
            if let Some(response) = settings_response {
                if !response.response.hovered() && ctx.input(|i| i.pointer.any_click()) {
                    self.show_settings = false;
                    self.font_search.clear();
                }
            }
        }
    }
}

// Implementation for UserInterface trait - this is a bridge to integrate with the existing architecture
impl UserInterface for GuiEditor {
    fn draw_rows(&self, _document: &Document, _offset: &Position) -> Result<(), std::io::Error> {
        // Handled by egui's update function
        Ok(())
    }
    
    fn draw_status_bar(&self, _document: &Document, _cursor_position: &Position, _status: &str) -> Result<(), std::io::Error> {
        // Handled by egui's update function
        Ok(())
    }
    
    fn draw_message_bar(&self, _message: &StatusMessage) -> Result<(), std::io::Error> {
        // Handled by egui's update function
        Ok(())
    }
    
    fn clear_screen(&self) -> Result<(), std::io::Error> {
        // Handled by egui's update function
        Ok(())
    }
    
    fn read_key(&self) -> Result<char, std::io::Error> {
        // Egui handles input events differently
        // This is a placeholder until we implement proper input handling
        Ok('\0')
    }
    
    fn cursor_position(&self, _position: &Position) -> Result<(), std::io::Error> {
        // Handled by egui's update function
        Ok(())
    }
    
    fn cursor_hide(&self) -> Result<(), std::io::Error> {
        // Not applicable in GUI mode
        Ok(())
    }
    
    fn cursor_show(&self) -> Result<(), std::io::Error> {
        // Not applicable in GUI mode
        Ok(())
    }
    
    fn size(&self) -> (usize, usize) {
        self.window_size
    }
}
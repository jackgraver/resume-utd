use crate::tui::components::{Sidebar, ContentPane};
use crate::data::DataManager;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::fs;
use std::path::Path;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

#[derive(Debug, Clone, PartialEq)]
pub enum MenuItem {
    PersonalInfo,
    Education,
    Experience,
    Projects,
    Skills,
    Export,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Sidebar,
    Content,
}

impl MenuItem {
    pub fn title(&self) -> &'static str {
        match self {
            MenuItem::PersonalInfo => "Personal Info",
            MenuItem::Education => "Education",
            MenuItem::Experience => "Experience",
            MenuItem::Projects => "Projects",
            MenuItem::Skills => "Skills",
            MenuItem::Export => "Export",
            MenuItem::Exit => "Exit Application",
        }
    }

    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem::PersonalInfo,
            MenuItem::Education,
            MenuItem::Experience,
            MenuItem::Projects,
            MenuItem::Skills,
            MenuItem::Export,
            MenuItem::Exit,
        ]
    }

    pub fn from_number(num: u8) -> Option<MenuItem> {
        match num {
            1 => Some(MenuItem::PersonalInfo),
            2 => Some(MenuItem::Education),
            3 => Some(MenuItem::Experience),
            4 => Some(MenuItem::Projects),
            5 => Some(MenuItem::Skills),
            6 => Some(MenuItem::Export),
            7 => Some(MenuItem::Exit),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub current_menu: MenuItem,
    pub focus: Focus,
    pub sidebar: Sidebar,
    pub content_pane: ContentPane,
    pub data_manager: DataManager,
    pub should_exit: bool,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_manager = DataManager::new("data/resume.json")?;
        Ok(Self {
            current_menu: MenuItem::PersonalInfo,
            focus: Focus::Sidebar,
            sidebar: Sidebar::new(),
            content_pane: ContentPane::new(),
            data_manager,
            should_exit: false,
        })
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), 
                Constraint::Percentage(75), 
            ])
            .split(frame.area());

        self.sidebar.render(frame, chunks[0], &self.current_menu, &self.focus);
        self.content_pane.render(frame, chunks[1], &self.current_menu, &self.focus, &self.data_manager);
    }

    pub fn next_menu(&mut self) {
        if self.focus == Focus::Sidebar {
            let items = MenuItem::all();
            if let Some(current_index) = items.iter().position(|item| item == &self.current_menu) {
                let next_index = (current_index + 1) % items.len();
                self.current_menu = items[next_index].clone();
            }
        } else {
            if self.content_pane.is_editing {
                self.content_pane.next_field();
            } else {
                match &self.current_menu {
                    MenuItem::Education | MenuItem::Experience | MenuItem::Projects => {
                        self.content_pane.next_entry(&self.data_manager, &self.current_menu);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn previous_menu(&mut self) {
        if self.focus == Focus::Sidebar {
            let items = MenuItem::all();
            if let Some(current_index) = items.iter().position(|item| item == &self.current_menu) {
                let prev_index = if current_index == 0 {
                    items.len() - 1
                } else {
                    current_index - 1
                };
                self.current_menu = items[prev_index].clone();
            }
        } else {
            if self.content_pane.is_editing {
                self.content_pane.previous_field();
            } else {
                match &self.current_menu {
                    MenuItem::Education | MenuItem::Experience | MenuItem::Projects => {
                        self.content_pane.previous_entry(&self.data_manager, &self.current_menu);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn select_current_menu(&mut self) {
        if self.focus == Focus::Sidebar {
            match &self.current_menu {
                MenuItem::Exit => {
                    self.exit();
                }
                MenuItem::Export => {
                    // self.focus = Focus::Content;
                    self.content_pane.trigger_export();
                }
                _ => {
                    self.focus = Focus::Content;
                    // self.previous_menu = 
                    self.content_pane.select_first_entry(&self.data_manager, &self.current_menu);
                }
            }
        } else {
            match &self.current_menu {
                MenuItem::Export => {
                    match self.export_resume() {
                        Ok(message) => {
                            self.content_pane.set_export_status(message);
                        }
                        Err(e) => {
                            self.content_pane.set_export_status(format!("Export failed: {}", e));
                        }
                    }
                }
                MenuItem::Education | MenuItem::Experience | MenuItem::Projects => {
                    if let Some(selected_idx) = self.content_pane.selected_entry {
                        let max_entries = match &self.current_menu {
                            MenuItem::Education => self.data_manager.resume.education.len(),
                            MenuItem::Experience => self.data_manager.resume.experience.len(),
                            MenuItem::Projects => self.data_manager.resume.projects.len(),
                            _ => 0,
                        };
                        
                        if selected_idx < max_entries {
                            self.content_pane.enter_entry_edit_mode(&self.current_menu, selected_idx, &self.data_manager);
                        } else {
                            self.content_pane.parent_menu = Some(self.current_menu.clone());
                            self.content_pane.enter_edit_mode(&self.current_menu);
                        }
                    } else {
                        self.content_pane.parent_menu = Some(self.current_menu.clone());
                        self.content_pane.enter_edit_mode(&self.current_menu);
                    }
                }
                _ => {
                    let needs_save = self.content_pane.handle_enter();
                    if needs_save {
                        if let Err(e) = self.content_pane.save_edited_data(&mut self.data_manager) {
                            eprintln!("Error saving data: {}", e);
                        } else {
                            if let Err(e) = self.data_manager.save() {
                                eprintln!("Error saving to file: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Sidebar => Focus::Content,
            Focus::Content => Focus::Sidebar,
        };
    }

    pub fn handle_text_input(&mut self, c: char) {
        if self.focus == Focus::Content {
            self.content_pane.handle_text_input(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.focus == Focus::Content {
            self.content_pane.handle_backspace();
        }
    }

    pub fn handle_number_input(&mut self, num: u8) {
        if self.focus == Focus::Sidebar {
            if let Some(menu_item) = MenuItem::from_number(num) {
                self.current_menu = menu_item;
            }
        }
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    pub fn export_resume(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file("resume", "templates/resume.tex.hbs")?;
        handlebars.register_helper("bold", Box::new(bold_helper));
        handlebars.register_helper("href", Box::new(href_helper));

        let tex_output = handlebars.render("resume", &self.data_manager.resume)?;

        let out_path = Path::new("output/resume.tex");
        fs::create_dir_all("output")?;
        fs::write(out_path, tex_output)?;

        Ok(format!("Generated {}", out_path.display()))
    }
}

fn bold_helper(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let text = param.value().as_str().unwrap_or("");
        out.write(&format!("\\textbf{{{}}}", text))?; 
    }
    Ok(())
}

fn href_helper(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let text = param.value().as_str().unwrap_or("");
        out.write(&format!("\\href{{{}}}{{GitHub Repo Link}}", text))?;
    }
    Ok(())
}

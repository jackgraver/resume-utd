use crate::tui::app::{MenuItem, Focus};
use crate::data::DataManager;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct ContentPane {
    content: String,
    current_field: usize,
    fields: Vec<String>,
    pub is_editing: bool,
    cursor_position: usize,
    pub selected_entry: Option<usize>,
    pub entry_type: Option<EntryType>,
    pub parent_menu: Option<MenuItem>,
    current_menu: Option<MenuItem>,
    pub export_status: Option<String>,
}

#[derive(Debug, Clone)]
pub enum EntryType {
    Education(usize),
    Experience(usize),
    Project(usize),
}

impl ContentPane {
    pub fn new() -> Self {
        Self {
            content: "Welcome to Resume Builder! Use arrow keys or j/k to navigate, Enter to select.".to_string(),
            current_field: 0,
            fields: Vec::new(),
            is_editing: false,
            cursor_position: 0,
            selected_entry: None,
            entry_type: None,
            parent_menu: None,
            current_menu: None,
            export_status: None,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, current_menu: &MenuItem, focus: &Focus, data_manager: &DataManager) {
        let title = format!("{}", current_menu.title());
        
        let content = if self.is_editing {
            self.render_editing_form(current_menu)
        } else {
            match current_menu {
                MenuItem::PersonalInfo => self.render_personal_info(data_manager, focus),
                MenuItem::Education => self.render_education(data_manager, focus),
                MenuItem::Experience => self.render_experience(data_manager, focus),
                MenuItem::Projects => self.render_projects(data_manager, focus),
                MenuItem::Skills => self.render_skills(focus),
                MenuItem::Export => self.render_export(focus),
                MenuItem::Exit => self.render_exit(),
            }
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(if focus == &Focus::Content {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn get_menu_description(&self, menu: &MenuItem) -> &'static str {
        match menu {
            MenuItem::PersonalInfo => "Enter your personal information",
            MenuItem::Education => "Add your educational background",
            MenuItem::Experience => "List your work experience",
            MenuItem::Projects => "Showcase your projects",
            MenuItem::Skills => "Highlight your skills",
            MenuItem::Export => "Export your resume",
            MenuItem::Exit => "Exit the application",
        }
    }

    fn render_personal_info(&self, data_manager: &DataManager, focus: &Focus) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
        ];

        lines.push(Line::from(format!("Name: {}", data_manager.resume.name)));
        lines.push(Line::from(format!("Contact: {}", data_manager.resume.contact)));
        if let Some(website) = &data_manager.resume.website {
            lines.push(Line::from(format!("Website: {}", website)));
        }
        lines.push(Line::from(""));
        if focus == &Focus::Content {
            lines.push(Line::from(Span::styled("Press Enter to edit", Style::default().fg(Color::Magenta))));
        }

        lines
    }

    fn render_education(&self, data_manager: &DataManager, focus: &Focus) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
        ];

        if data_manager.resume.education.is_empty() {
            lines.push(Line::from("No education entries found."));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Press Enter to add education", Style::default().fg(Color::Yellow))));
        } else {
            for (i, education) in data_manager.resume.education.iter().enumerate() {
                let is_selected = self.selected_entry == Some(i);
                let style = if is_selected && focus == &Focus::Content {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                lines.push(Line::from(Span::styled(
                    format!("{}. {}", i + 1, education.name),
                    style,
                )));
                
                if is_selected && focus == &Focus::Content {
                    lines.push(Line::from(format!("   Degree: {}", education.degree)));
                    lines.push(Line::from(format!("   Years: {}", education.years)));
                    for point in &education.points {
                        lines.push(Line::from(format!("   • {}", point)));
                    }
                    lines.push(Line::from(""));
                }
            }
            
            
            let add_new_selected = self.selected_entry == Some(data_manager.resume.education.len());
            let add_new_style = if add_new_selected {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            };
            
            if focus == &Focus::Content {
                lines.push(Line::from(Span::styled(
                    format!("{}. + Add new education", data_manager.resume.education.len() + 1),
                    add_new_style,
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Use ↑/↓ to select, Enter to edit", Style::default().fg(Color::Cyan))));
            }
            
        }

        lines
    }

    fn render_experience(&self, data_manager: &DataManager, focus: &Focus) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
        ];

        if data_manager.resume.experience.is_empty() {
            lines.push(Line::from("No experience entries found."));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Press Enter to add experience", Style::default().fg(Color::Yellow))));
        } else {
            for (i, experience) in data_manager.resume.experience.iter().enumerate() {
                let is_selected = self.selected_entry == Some(i);
                let style = if is_selected && focus == &Focus::Content {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                lines.push(Line::from(Span::styled(
                    format!("{}. {} at {}", i + 1, experience.position, experience.company),
                    style,
                )));
                
                if is_selected && focus == &Focus::Content {
                    lines.push(Line::from(format!("   Company: {}", experience.company)));
                    lines.push(Line::from(format!("   Position: {}", experience.position)));
                    lines.push(Line::from(format!("   Duration: {}", experience.years)));
                    for point in &experience.points {
                        lines.push(Line::from(format!("   • {}", point)));
                    }
                    lines.push(Line::from(""));
                }
            }
            
            
            let add_new_selected = self.selected_entry == Some(data_manager.resume.experience.len());
            let add_new_style = if add_new_selected {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            };
            
            if focus == &Focus::Content {
                lines.push(Line::from(Span::styled(
                    format!("{}. + Add new experience", data_manager.resume.experience.len() + 1),
                    add_new_style,
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Use ↑/↓ to select, Enter to edit", Style::default().fg(Color::Cyan))));
            }
        }
        lines
    }

    fn render_projects(&self, data_manager: &DataManager, focus: &Focus) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
        ];

        if data_manager.resume.projects.is_empty() {
            lines.push(Line::from("No project entries found."));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Press Enter to add project", Style::default().fg(Color::Yellow))));
        } else {
            for (i, project) in data_manager.resume.projects.iter().enumerate() {
                let is_selected = self.selected_entry == Some(i);
                let style = if is_selected && focus == &Focus::Content {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                lines.push(Line::from(Span::styled(
                    format!("{}. {}", i + 1, project.name),
                    style,
                )));
                
                if is_selected && focus == &Focus::Content {
                    if let Some(link) = &project.link {
                        lines.push(Line::from(format!("   Link: {}", link)));
                    }
                    for point in &project.points {
                        lines.push(Line::from(format!("   • {}", point)));
                    }
                    lines.push(Line::from(""));
                }
            }
            
            let add_new_selected = self.selected_entry == Some(data_manager.resume.projects.len());
            let add_new_style = if add_new_selected {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            };
            
            if focus == &Focus::Content {
                lines.push(Line::from(Span::styled(
                    format!("{}. + Add new project", data_manager.resume.projects.len() + 1),
                    add_new_style,
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Use ↑/↓ to select, Enter to edit", Style::default().fg(Color::Cyan))));
            }
        }

        lines
    }

    fn render_skills(&self, focus: &Focus) -> Vec<Line> {
        vec![
            Line::from(""),
            Line::from("List your technical skills:"),
            Line::from("• Programming Languages: [List languages]"),
            Line::from("• Frameworks: [Web frameworks, libraries]"),
            Line::from("• Tools: [Development tools, IDEs]"),
            Line::from("• Databases: [Database technologies]"),
            Line::from("• Other: [Certifications, soft skills]"),
            Line::from(""),
            if focus == &Focus::Content {
                Line::from(Span::styled("Press Enter to add/edit skills", Style::default().fg(Color::Yellow)))
            } else {
                Line::from("")
            }
        ]
    }

    fn render_export(&self, focus: &Focus) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
            Line::from("Export your resume to LaTeX format"),
            Line::from(""),
            Line::from("Export location: ./output/resume.tex"),
            Line::from(""),
        ];

        if let Some(status) = &self.export_status {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Status: {}", status),
                Style::default().fg(Color::Green),
            )));
        } else {
            if focus == &Focus::Content {
                lines.push(Line::from(Span::styled("Press Enter to export resume", Style::default().fg(Color::Yellow))));
            }
        }

        lines
    }

    fn render_exit(&self) -> Vec<Line> {
        vec![
            Line::from(""),
            Line::from(Span::styled("Press Enter to exit", Style::default().fg(Color::Red))),
        ]
    }

    pub fn enter_edit_mode(&mut self, menu: &MenuItem) {
        self.is_editing = true;
        self.current_field = 0;
        self.cursor_position = 0;
        self.fields = self.get_default_fields(menu);
        self.selected_entry = None;
        self.entry_type = None;
        self.current_menu = Some(menu.clone());
    }

    pub fn enter_entry_edit_mode(&mut self, menu: &MenuItem, entry_idx: usize, data_manager: &DataManager) {
        self.is_editing = true;
        self.current_field = 0;
        self.cursor_position = 0;
        self.selected_entry = Some(entry_idx);
        self.current_menu = Some(menu.clone());
        
        
        self.fields = match menu {
            MenuItem::Education => {
                if let Some(education) = data_manager.resume.education.get(entry_idx) {
                    self.entry_type = Some(EntryType::Education(entry_idx));
                    vec![
                        education.name.clone(),
                        education.degree.clone(),
                        education.years.clone(),
                        education.points.join("; "),
                    ]
                } else {
                    self.get_default_fields(menu)
                }
            }
            MenuItem::Experience => {
                if let Some(experience) = data_manager.resume.experience.get(entry_idx) {
                    self.entry_type = Some(EntryType::Experience(entry_idx));
                    vec![
                        experience.company.clone(),
                        experience.position.clone(),
                        experience.years.clone(),
                        experience.points.join("; "),
                    ]
                } else {
                    self.get_default_fields(menu)
                }
            }
            MenuItem::Projects => {
                if let Some(project) = data_manager.resume.projects.get(entry_idx) {
                    self.entry_type = Some(EntryType::Project(entry_idx));
                    vec![
                        project.name.clone(),
                        project.link.clone().unwrap_or_default(),
                        project.points.join("; "),
                    ]
                } else {
                    self.get_default_fields(menu)
                }
            }
            _ => self.get_default_fields(menu),
        };
    }

    pub fn next_field(&mut self) {
        if self.is_editing && self.current_field < self.fields.len() - 1 {
            self.current_field += 1;
            self.cursor_position = self.fields[self.current_field].len();
        }
    }

    pub fn previous_field(&mut self) {
        if self.is_editing && self.current_field > 0 {
            self.current_field -= 1;
            self.cursor_position = self.fields[self.current_field].len();
        }
    }

    pub fn select_first_entry(&mut self, data_manager: &DataManager, menu: &MenuItem) {
        let max_entries = match menu {
            MenuItem::Education => data_manager.resume.education.len(),
            MenuItem::Experience => data_manager.resume.experience.len(),
            MenuItem::Projects => data_manager.resume.projects.len(),
            _ => 0,
        };

        if max_entries > 0 {
            self.selected_entry = Some(0);
        } else {
            self.selected_entry = None;
        }
    }

    pub fn next_entry(&mut self, data_manager: &DataManager, menu: &MenuItem) {
        let max_entries = match menu {
            MenuItem::Education => data_manager.resume.education.len() + 1, 
            MenuItem::Experience => data_manager.resume.experience.len() + 1,
            MenuItem::Projects => data_manager.resume.projects.len() + 1,
            _ => 0,
        };

        if max_entries > 0 {
            self.selected_entry = Some(match self.selected_entry {
                Some(idx) => (idx + 1) % max_entries,
                None => 0,
            });
        }
    }

    pub fn previous_entry(&mut self, data_manager: &DataManager, menu: &MenuItem) {
        let max_entries = match menu {
            MenuItem::Education => data_manager.resume.education.len() + 1, 
            MenuItem::Experience => data_manager.resume.experience.len() + 1,
            MenuItem::Projects => data_manager.resume.projects.len() + 1,
            _ => 0,
        };

        if max_entries > 0 {
            self.selected_entry = Some(match self.selected_entry {
                Some(idx) => if idx == 0 { max_entries - 1 } else { idx - 1 },
                None => max_entries - 1,
            });
        }
    }

    pub fn handle_enter(&mut self) -> bool {
        if self.is_editing {
            if self.current_field < self.fields.len() - 1 {
                self.next_field();
                false 
            } else {
                
                self.is_editing = false;
                self.selected_entry = None;
                self.entry_type = None;
                true 
            }
        } else {
            false
        }
    }

    pub fn save_edited_data(&mut self, data_manager: &mut DataManager) -> Result<(), String> {
        if !self.is_editing {
            return Ok(());
        }

        match &self.entry_type {
            Some(EntryType::Education(idx)) => {
                if self.fields.len() >= 4 {
                    let points: Vec<String> = self.fields[3]
                        .split(';')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    
                    let education = crate::data::manager::Education {
                        name: self.fields[0].clone(),
                        degree: self.fields[1].clone(),
                        years: self.fields[2].clone(),
                        points,
                    };
                    
                    data_manager.update_education(*idx, education)?;
                }
            }
            Some(EntryType::Experience(idx)) => {
                if self.fields.len() >= 4 {
                    let points: Vec<String> = self.fields[3]
                        .split(';')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    
                    let experience = crate::data::manager::Experience {
                        company: self.fields[0].clone(),
                        position: self.fields[1].clone(),
                        years: self.fields[2].clone(),
                        points,
                    };
                    
                    data_manager.update_experience(*idx, experience)?;
                }
            }
            Some(EntryType::Project(idx)) => {
                if self.fields.len() >= 3 {
                    let points: Vec<String> = self.fields[2]
                        .split(';')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    
                    let project = crate::data::manager::Project {
                        name: self.fields[0].clone(),
                        link: if self.fields[1].is_empty() { None } else { Some(self.fields[1].clone()) },
                        points,
                    };
                    
                    data_manager.update_project(*idx, project)?;
                }
            }
            None => {
                
                match self.get_current_menu_type() {
                    Some(MenuItem::Education) => {
                        if self.fields.len() >= 4 {
                            let points: Vec<String> = self.fields[3]
                                .split(';')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            
                            let education = crate::data::manager::Education {
                                name: self.fields[0].clone(),
                                degree: self.fields[1].clone(),
                                years: self.fields[2].clone(),
                                points,
                            };
                            
                            data_manager.add_education(education);
                        }
                    }
                    Some(MenuItem::Experience) => {
                        if self.fields.len() >= 4 {
                            let points: Vec<String> = self.fields[3]
                                .split(';')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            
                            let experience = crate::data::manager::Experience {
                                company: self.fields[0].clone(),
                                position: self.fields[1].clone(),
                                years: self.fields[2].clone(),
                                points,
                            };
                            
                            data_manager.add_experience(experience);
                        }
                    }
                    Some(MenuItem::Projects) => {
                        if self.fields.len() >= 3 {
                            let points: Vec<String> = self.fields[2]
                                .split(';')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            
                            let project = crate::data::manager::Project {
                                name: self.fields[0].clone(),
                                link: if self.fields[1].is_empty() { None } else { Some(self.fields[1].clone()) },
                                points,
                            };
                            
                            data_manager.add_project(project);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn get_current_menu_type(&self) -> Option<MenuItem> {
        self.current_menu.clone()
    }

    pub fn handle_text_input(&mut self, c: char) {
        if self.is_editing && self.current_field < self.fields.len() {
            self.fields[self.current_field].insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.is_editing && self.current_field < self.fields.len() && self.cursor_position > 0 {
            self.fields[self.current_field].remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    fn get_default_fields(&self, menu: &MenuItem) -> Vec<String> {
        match menu {
            MenuItem::PersonalInfo => vec![
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
            ],
            MenuItem::Education => vec![
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
            ],
            MenuItem::Experience => vec![
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
            ],
            MenuItem::Projects => vec![
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
            ],
            MenuItem::Skills => vec![
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
            ],
            _ => vec!["".to_string()],
        }
    }

    fn render_editing_form(&self, menu: &MenuItem) -> Vec<Line> {
        let field_labels = self.get_field_labels(menu);
        let mut lines = vec![
            Line::from("Edit Mode - Use ↑/↓ to navigate fields, type to edit, Enter to save"),
            Line::from(""),
        ];

        for (i, (label, value)) in field_labels.iter().zip(self.fields.iter()).enumerate() {
            let is_current = i == self.current_field;
            let style = if is_current {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let cursor_char = if is_current && self.cursor_position < value.len() {
                "|"
            } else if is_current {
                "_"
            } else {
                " "
            };

            let display_value = if value.is_empty() {
                format!("[Enter {}]", label)
            } else {
                value.clone()
            };

            lines.push(Line::from(Span::styled(
                format!("{}: {}{}", label, display_value, cursor_char),
                style,
            )));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press Tab to switch back to sidebar, Enter to save",
            Style::default().fg(Color::Cyan),
        )));

        lines
    }

    fn get_field_labels(&self, menu: &MenuItem) -> Vec<&'static str> {
        match menu {
            MenuItem::PersonalInfo => vec!["Name", "Email", "Phone", "Location", "LinkedIn", "GitHub"],
            MenuItem::Education => vec!["Institution", "Degree", "Years", "Points (semicolon separated)"],
            MenuItem::Experience => vec!["Company", "Position", "Duration", "Points (semicolon separated)"],
            MenuItem::Projects => vec!["Project Name", "Link", "Points (semicolon separated)"],
            MenuItem::Skills => vec!["Programming Languages", "Frameworks", "Tools", "Databases", "Other"],
            _ => vec!["Field"],
        }
    }

    pub fn show_export(&mut self) {
        self.content = "Exporting resume...".to_string();
    }

    pub fn show_form(&mut self, menu: &MenuItem) {
        self.content = format!("Editing {} form...", menu.title());
    }

    pub fn trigger_export(&mut self) {
        self.export_status = Some("Exporting...".to_string());
    }

    pub fn set_export_status(&mut self, status: String) {
        self.export_status = Some(status);
    }

    pub fn clear_export_status(&mut self) {
        self.export_status = None;
    }
}

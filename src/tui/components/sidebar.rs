use crate::tui::app::{MenuItem, Focus};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

#[derive(Debug)]
pub struct Sidebar {
    items: Vec<MenuItem>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            items: MenuItem::all(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, current_menu: &MenuItem, focus: &Focus) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let is_selected = item == current_menu;
                let is_focused = focus == &Focus::Sidebar;
                let menu_number = index + 1;
                
                let style = if is_selected {
                    if is_focused {
                        Style::default()
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
                    } else {
                        Style::default()
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD)
                    }
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(Line::from(Span::styled(
                    format!("{} {}", menu_number, item.title()),
                    style,
                )))
            })
            .collect();

        let title = if focus == &Focus::Sidebar {
            "Resume Builder"
        } else {
            "Resume Builder"
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(if focus == &Focus::Sidebar {
                        Style::default().fg(Color::Magenta)
                    } else {
                        Style::default().fg(Color::White)
                    }),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(list, area);
    }
}

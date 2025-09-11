use crate::tui::app::{AppState, Focus};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = AppState::new().map_err(|e| {
        eprintln!("Failed to load resume data: {}", e);
        io::Error::new(io::ErrorKind::Other, format!("{}", e))
    })?;

    let result = run_event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| app.render(frame))?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {
                        app.previous_menu();
                    }
                    KeyCode::Down => {
                        app.next_menu();
                    }
                    KeyCode::Enter => { // going into a sidebar option or content pane option
                        app.select_current_menu();
                    }
                    KeyCode::Esc => { // either sub-content -> content or content -> sidebar
                        match app.focus {
                            Focus::Sidebar => (),
                            Focus::Content => {
                                if app.content_pane.is_editing {
                                    app.content_pane.is_editing = false;
                                    app.content_pane.selected_entry = None;
                                    app.content_pane.entry_type = None;
                                    if let Some(parent) = &app.content_pane.parent_menu {
                                        app.current_menu = parent.clone();
                                    } else {
                                        app.select_current_menu();
                                    }
                                } else {
                                    app.focus = Focus::Sidebar;
                                }
                            }
                        }
                    }

                    KeyCode::Tab => {
                        let new_focus = app.switch_focus();
                        match app.focus {
                            Focus::Content => {
                                app.next_menu();
                            }
                            _ => {}
                        }

                    }
                    KeyCode::Backspace => {
                        app.handle_backspace();
                    }
                    KeyCode::Char(c) => {
                        if app.content_pane.is_editing {
                            app.handle_text_input(c);
                        } else {
                            if let Some(digit) = c.to_digit(10) {
                                if digit >= 1 && digit <= 8 {
                                    app.handle_number_input(digit as u8);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if app.should_exit {
            break;
        }
    }

    Ok(())
}

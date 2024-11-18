use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io::{self};

pub struct PasswordManagerApp {
    passwords: Vec<(String, String)>, // Stores (Service, Password)
    selected: usize,
}

impl PasswordManagerApp {
    pub fn new() -> PasswordManagerApp {
        PasswordManagerApp {
            passwords: vec![
                ("Email".to_string(), "password123".to_string()),
                ("Bank".to_string(), "securepass456".to_string()),
                ("Social Media".to_string(), "myp@ssw0rd!".to_string()),
            ],
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.passwords.len();
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.passwords.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn add_password(&mut self, service: String, password: String) {
        self.passwords.push((service, password));
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: PasswordManagerApp) -> io::Result<()> {

    loop {
        terminal.draw(|f| {


            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let title = Paragraph::new(Span::styled(
                "Password Manager",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ))
            .block(Block::default().borders(Borders::ALL).title("Title"));
            f.render_widget(title, chunks[0]);

            let password_list: Vec<ListItem> = app
                .passwords
                .iter()
                .enumerate()
                .map(|(i, (service, password))| {
                    let style = if i == app.selected {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(service.clone(), style),
                        Span::raw(": "),
                        Span::raw(password.clone()),
                    ]))
                })
                .collect();

            let list = List::new(password_list)
                .block(Block::default().borders(Borders::ALL).title("Passwords"))
                .highlight_style(Style::default().fg(Color::LightYellow));
            f.render_widget(list, chunks[1]);

            let footer = Paragraph::new("Use Up/Down to navigate, 'a' to add, 'q' to quit")
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Help"));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    terminal.clear()?;
                    return Ok(())},
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Char('a') => {
                    // Simulate adding a new password
                    app.add_password("New Service".to_string(), "new_password".to_string());
                }
                _ => {}
            }
        }
    }
}
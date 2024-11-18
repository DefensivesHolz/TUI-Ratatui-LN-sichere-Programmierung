use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;


pub struct LoginApp {
    username: String,
    password: String,
    current_input: InputMode,
}

enum InputMode {
    Username,
    Password,
}

impl LoginApp {
    pub fn new() -> Self {
        LoginApp {
            username: String::new(),
            password: String::new(),
            current_input: InputMode::Username,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let username = Paragraph::new(Line::from(vec![Span::raw(format!(
                    "Username: {}",
                    self.username
                ))]))
                .block(Block::default().borders(Borders::ALL).title("Login"));
                f.render_widget(username, chunks[0]);

                let password_display = "*".repeat(self.password.len());
                let password = Paragraph::new(Line::from(vec![Span::raw(format!(
                    "Password: {}",
                    password_display
                ))]))
                .block(Block::default().borders(Borders::ALL).title("Password"));
                f.render_widget(password, chunks[1]);

                let help = Paragraph::new(Text::from("Press Enter to login, Tab to switch input, Esc to quit"))
                    .style(Style::default().fg(Color::White))
                    .block(Block::default().borders(Borders::ALL).title("Help"));
                f.render_widget(help, chunks[3]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => match self.current_input {
                        InputMode::Username => self.username.push(c),
                        InputMode::Password => self.password.push(c),
                    },
                    KeyCode::Backspace => match self.current_input {
                        InputMode::Username => {
                            self.username.pop();
                        }
                        InputMode::Password => {
                            self.password.pop();
                        }
                    },
                    KeyCode::Tab => {
                        self.current_input = match self.current_input {
                            InputMode::Username => InputMode::Password,
                            InputMode::Password => InputMode::Username,
                        };
                    }
                    KeyCode::Enter => {
                        // Simple authentication logic
                        if self.username == "admin" && self.password == "password123" {
                            return Ok(true);
                        }
                        // Reset input on failed login
                        self.username.clear();
                        self.password.clear();
                    }
                    KeyCode::Esc => {terminal.clear()?;
                        return Ok(false)},
                    _ => {}
                }
            }
        }
    }
}

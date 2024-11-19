//menu for choosing which App to come next


use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, BorderType},
    Terminal,
};
use std::io::{self};

pub struct MenuApp {
    options: Vec<String>,
    choice: u8,
}

impl MenuApp {
    pub fn new() -> Self {
        MenuApp {
            options: vec![
                "1. See Passwords".to_string(),
                "2. Change Passwords".to_string(),
            ],
            choice: 0, // Default value
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<u8> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(4)
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


                let header = Paragraph::new("Menu")
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().light_green())
                );
                f.render_widget(header, chunks[0]);

                let p1 = Paragraph::new("1. See Passwords")
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            
                            .border_type(BorderType::Rounded),
                    );
                f.render_widget(p1, chunks[1]);

                let p2 = Paragraph::new("2. Change Passwords")
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    );
                f.render_widget(p2, chunks[2]);

                let input_paragraph = Paragraph::new(Line::from(vec![Span::raw(format!(
                    "Choice: {}",
                    self.choice,
                ))]))
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Enter your choice (1 or 2 or ...) or press Esc to quit"),
                );
                f.render_widget(input_paragraph, chunks[3]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('1') => self.choice = 1,
                    KeyCode::Char('2') => self.choice = 2,
                    KeyCode::Backspace => self.choice = 0, // Default value
                    KeyCode::Enter => return Ok(self.choice),
                    KeyCode::Esc => {
                        terminal.clear()?;
                        return Ok(220); // Default value after exit
                    }
                    _ => {}
                }
            }
        }
    }
}

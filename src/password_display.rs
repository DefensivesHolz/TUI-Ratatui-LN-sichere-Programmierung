use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, BorderType},
    Terminal,
};
use std::{io::{self, Error}, rc::Rc};

pub struct Password_Info{
    id : u8,
    url: String, 
    name : String,
    password : String,
    last_modified : String,
}

pub struct Password_Application{
    password : Vec<Password_Info>,
    selected : usize,
}

impl Password_Application{
    pub fn new()->Self{
        
        Password_Application{
            password : vec![Password_Info{
                id: 0,
                url: "new_password".to_string(),
                name: "name".to_string(),
                password: "password".to_string(),
                last_modified: "01.01.01".to_string(),
            },],
        selected : 0,
        }
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.password.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.password.len();
    }
    pub fn draw_line(&mut self, passwird_info : Password_Info, i : usize, frame: &mut Frame<'_>, layout : Rc<[Rect]>){
        frame.render_widget(Paragraph::new(self.password[i].url.as_str()), layout[0]);
        frame.render_widget(Paragraph::new(self.password[i].name.as_str()), layout[1]);
        frame.render_widget(Paragraph::new(self.password[i].last_modified.as_str()),layout[2]);
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>)->Result<(), Error>{
        terminal.clear()?;
        loop{
            terminal.draw(|frame|{
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            )
            .split(frame.area());

        let line_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),//URL
                Constraint::Percentage(30),//Name
                Constraint::Percentage(40), // Last modified
            ])
            .split(outer_layout[0]);
        
             // Line ausgeben

            frame.render_widget(Paragraph::new(self.password[0].url.as_str()), line_layout[0]);
            frame.render_widget(Paragraph::new(self.password[0].name.as_str()),  line_layout[1]);
            frame.render_widget(Paragraph::new(self.password[0].last_modified.as_str()), line_layout[2]);

            })?;
        

        if let Event::Key(key) = event::read()?{
            match key.code {
                KeyCode::Up => self.previous(),
                KeyCode::Esc => {
                    terminal.clear()?;
                    return Ok(());
                },

                KeyCode::Down => self.next(),
                _ => {}
            }
            
        }
    }

}

}
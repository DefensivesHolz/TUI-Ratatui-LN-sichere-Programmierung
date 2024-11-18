mod status;
mod login;
mod menu;
mod password_manager;
use crossterm::terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use ratatui::Terminal;


fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal::enable_raw_mode()?;
    terminal.clear()?;

    let mut login_app = login::LoginApp::new();
    let res1 = login_app.run(&mut terminal);

    terminal.clear()?;

    if res1?{
        terminal.clear()?;
        let mut password_manager_app = password_manager::PasswordManagerApp::new();
        password_manager::run_app(&mut terminal, password_manager_app)?;
    }
    else{
        println!("Authentication failed or user quit!");
    }

    terminal::disable_raw_mode()?;

    terminal.show_cursor()?;


    //if let Err(err) = res {
    //    eprintln!("{:?}", err);
    //}

    Ok(())
}

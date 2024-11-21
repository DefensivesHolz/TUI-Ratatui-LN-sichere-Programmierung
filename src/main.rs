mod password_display;
mod status;
mod login;
mod menu;
mod password_manager;
use crossterm::terminal;
use menu::MenuApp;
use ratatui::backend::{self, CrosstermBackend};
use std::{fmt::Error, io::{self, Stdout}};
use ratatui::Terminal;


/*fn run_login(mut terminal: Terminal<CrosstermBackend<Stdout>>)->(login::LoginApp, Result<bool,std::io::Error>){
    let mut login_app = login::LoginApp::new();
    let res1 = login_app.run(&mut terminal);
    return (login_app, res1);
}
    */

fn run_menu(mut terminal : Terminal<CrosstermBackend<Stdout>>)->(MenuApp, Result<u8,std::io::Error>){
    let mut menu_app = menu::MenuApp::new();
    let res2 = menu_app.run(&mut terminal);
    return (menu_app, res2);
}

fn main() -> io::Result<()> {
    let mut app_state = status::App::new();
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal::enable_raw_mode()?;
    terminal.clear()?;


    /*let login_return = run_login(terminal);
    let login_app = login_return.0;
    let res1 = login_return.1;
    */

    //do login
    let mut login_app = login::LoginApp::new();
    let res1 = login_app.run(&mut terminal);

    terminal.clear()?;

    let mut password_display1 = password_display::Password_Application::new();
    password_display1.run(&mut terminal)?;
/* 

    let mut menu_app = menu::MenuApp::new();
    let res2 = menu_app.run(&mut terminal);

    match res2{
        Ok(1) => {
            app_state.set_status(status::App_State::PasswordManagerApp);
            if res1?{
                terminal.clear()?;
                let mut password_manager_app = password_manager::PasswordManagerApp::new();
                let action = password_manager::run_app(&mut terminal, password_manager_app)?;
                terminal.clear()?;
                if action == 'q'{
                    terminal::disable_raw_mode()?;
                    terminal.show_cursor()?;
                }
                if action == 'c'{
                    
                }
            }
            else{
                println!("Authentication failed or user quit!");
            }
        },

        Ok(_) => {terminal.clear()?;
            println!("Invalid option selected");}
        Err(_) => {terminal.clear()?;
            println!("An error ocurred!");},
        }
    



        terminal::disable_raw_mode()?;
        terminal.show_cursor()?;



    */



    Ok(())
}

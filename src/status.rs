

use crate::login::LoginApp;
use crate::password_manager::PasswordManagerApp;


pub enum App_State{
    LoginApp,
    PasswordManagerApp,
    MenuApp,
}

pub struct App{
    status: App_State,
}

impl App{
    pub fn new()->Self{
        App{
        status: App_State::LoginApp,
    }
    }

    pub fn get_status(&self)->&App_State{
        &self.status
    }

    pub fn set_status(&mut self, status_new : App_State){
        self.status = status_new;
    }

}
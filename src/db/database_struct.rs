use serde::{Serialize, Deserialize};
use crate::db::crud::*;
use color_eyre::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub  server_address: String,
    pub  username: String,
    pub  token: String,
    pub  is_default_usr: bool,
    pub  name_selected_lib: String,
    pub  id_selected_lib: String,
    pub  is_loop_break: String,
    pub  is_vlc_launched_first_time: String,
}

pub struct Database  {
    pub users: Vec<User>,
    pub default_usr: Vec<String>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        // open db and create table if there is none
        let _ = init_db();

        // init empty Vec<User> for future add of users
        let users: Vec<User> = vec![];

        // retrieve default user
        let mut default_usr: Vec<String> = Vec::new();

        if let Ok(mut result) = select_default_usr() {
            default_usr = result;
        }

        // init should_exit
        let should_exit = false;

        Ok(Self {
            users,
            default_usr,
        })
    }
}


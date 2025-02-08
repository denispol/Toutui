mod app;
mod config;
mod api;
mod ui;
mod player;
mod logic;
mod db;
use app::App;
use color_eyre::Result;
use tokio::io::{self, AsyncBufReadExt};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tokio::time::sleep;
use serde::{Serialize, Deserialize};
use crate::db::db::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub  server_adress: String,
    pub  username: String,
    pub  password: String,
    pub  token: String,
    pub  is_default_usr: bool,
    pub  name_selected_lib: String,
    pub  id_selected_lib: String,
}

pub struct Database  {
   pub users: Vec<User>,
   pub default_usr: Vec<String>,
}


#[tokio::main]
async fn main() -> Result<()> {

    impl Database {
        pub async fn new() -> Result<Self> {
            // db test
            let _ = db();

            // init empty Vec<User> for future addition of users
            let users: Vec<User> = vec![];
//            let users = vec![
//                User {
//                    server_adress: "https://nuagemagique.duckdns.org".to_string(),
//                    username: "luc".to_string(),
//                    password: "acac".to_string(),
//                    token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI3MzBlNDIzYS1mY2ZhLTQ0MjQtYmY0Yi03YWI5NGJmODIzZGYiLCJ1c2VybmFtZSI6Imx1YyIsImlhdCI6MTczMjQ1OTE4NX0.hvkolaE_yCNqgfvdqsycWC981ybwNME8GzkH6s5XOMU".to_string(),
//                        is_default_usr: true,
//                        name_selected_lib: "LeNuageMagique".to_string(),
//                        id_selected_lib: "5d80300e-e228-402e-9b6e-1356ff1f4243".to_string(),
//                },
//                User {
//                    server_adress: "https://example.com".to_string(),
//                    username: "alice".to_string(),
//                    password: "securepassword".to_string(),
//                    token: "123".to_string(),
//                    is_default_usr: false,
//                    name_selected_lib: "Library2".to_string(),
//                    id_selected_lib: "12345678-aaaa-bbbb-cccc-1356ff1f4243".to_string(),
//                },
//            ];
//
//            // insert users in db :
//            db_insert_usr(&users);

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


    // Init app
    color_eyre::install()?;
    let mut app = App::new().await?;
    let mut terminal = ratatui::init();

    loop {
        // Refresh app
        let result = app.run(&mut terminal);

        if let Err(e) = result {
            eprintln!("Error running the app: {:?}", e);
        }

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('R') => {
                       // println!("Refreshing app...");
                        
                        app = App::new().await?; 
                    }
                    crossterm::event::KeyCode::Char('Q') | KeyCode::Esc => {
                       // println!("Exiting app...");
                        break;
                    }
                    _ => {}
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    ratatui::restore();
    Ok(())
}

mod login_app;
mod app;
mod config;
mod api;
mod ui;
mod player;
mod logic;
mod db;
mod utils;

use login_app::AppLogin;
use app::App;
use crate::db::database_struct::Database;
use color_eyre::Result;
use std::time::Duration;
use crossterm::event::{self, KeyCode};
use std::io::stdout;
use crate::utils::pop_up_message::*;
use crate::utils::logs::*;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<()> {

    // this function allow to write all the logs in a file 
    setup_logs().expect("Failed to execute logger");

    // set dotenv to ~/.config.toutui/.env (dotenv will be use in `encrypt_token.rs`)
    let home_dir = dirs::home_dir().expect("Unable to retrieve home directory");
    let env_path = home_dir.join(".config").join("toutui").join(".env");
    dotenv::from_filename(&env_path.clone()).ok();

    // Init database
    let mut database = Database::new().await?;
    let mut database_ready = false;

    // Wait for the database to be ready, waiting for the user to enter their credentials
    loop {
        database = Database::new().await?;
        if database.default_usr.is_empty() {
            let app_login = AppLogin::new().await?;
            let terminal = ratatui::init();
            let app_result = app_login.run(terminal);
            app_result;
            // Process login result here
            // Wait for 1 second before checking again
            // If database is reinit to quickly before `auth_process.rs` is finished
            // it can be buggy and mark as failed. Maybe add more time to be sure (like 6 sec).
            // But normally, even it's failed, data are written in db. It will work at the second
            // attempt...
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            // If the database is ready, exit the loop
            print!("\x1B[2J\x1B[1;1H"); // clear all stdout (avoid to sill have the previous print when the app is launched)
            database_ready = true;
            info!("Database ready");
            break;
        }
    }

    // Once the database is ready, initialize the app
    if database_ready {
        let mut app = App::new().await?;
        let mut terminal = ratatui::init();

        // Running the app in a loop
        loop {
            // If `app` variable is reinitialized below (`app = App::new().await?`), it will be taken into account and data will be refreshed
            // Otherwise, the current `app` variable will still be used.
            let result = app.run(&mut terminal);

            if let Err(e) = result {
                eprintln!("Error running the app: {:?}", e);
                error!("Error running the app: {:?}", e);
            }

            // Checking if any key is pressed (waiting for events with a 200ms delay here)
            if crossterm::event::poll(Duration::from_millis(200))? {
                if let event::Event::Key(key) = crossterm::event::read()? {
                    match key.code {
                        // If the 'R' key is pressed, refresh the app
                        KeyCode::Char('R') => {
                            // pop up message
                            let mut stdout = stdout();
                            let _ = clear_message(&mut stdout, 3); // clear a message, if any, before print the message bellow
                            let _ = pop_message(&mut stdout, 3, "Refreshing app...");
                            // Reinitialize app to refresh
                            app = App::new().await?; 
                            // clear message above
                            let _ = clear_message(&mut stdout, 3);
                        }
                        // If 'Q' or 'Esc' is pressed, exit the app
                        KeyCode::Char('Q') | KeyCode::Esc => {
                            println!("Exiting app...");
                            break;
                        }
                        _ => {}
                    }
                }
            }

            // Short pause between event checks
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    // Restore the terminal state before exiting the application
    ratatui::restore();
    Ok(())
}

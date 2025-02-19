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
use std::process;
use app::App;
use crate::db::database_struct::Database;
use color_eyre::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use std::io::{stdout, Write};
use crossterm::{cursor, execute, terminal, ExecutableCommand};
use crate::utils::pop_up_message::*;


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    // Initial database creation
    let mut database = Database::new().await?;
    let mut database_ready = false;

    // Wait for the database to be ready, waiting for the user to enter their credentials
    loop {
        database = Database::new().await?;
        if database.default_usr.is_empty() {
            print!("Authentification failed");
            let app_login = AppLogin::new().await?;
            let terminal = ratatui::init();
            let app_result = app_login.run(terminal);
            app_result; // Process login result here
            // Wait for 1 second before checking again
            // If database is reinit to quickly before `auth_process.rs` is finished
            // it can be buggy and mark as failed. Maybe add more time to be sure (like 6 sec).
            // But normally, even it's failed, data are written in db. It will work at the second
            // attempt...
            tokio::time::sleep(Duration::from_secs(1)).await;

            // Reload or update the database
        } else {
            // If the database is ready, exit the loop
            print!("\x1B[2J\x1B[1;1H"); // clear all stdout (avoid to sill have the previous print
                                        // when the app is launched)
            database_ready = true;
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
            }

            // Checking if any key is pressed (waiting for events with a 200ms delay here)
            if crossterm::event::poll(Duration::from_millis(200))? {
                if let event::Event::Key(key) = crossterm::event::read()? {
                    match key.code {
                        // If the 'R' key is pressed, refresh the app
                        KeyCode::Char('R') => {
                            // pop up message
                            let mut stdout = stdout();
                            let (_cols, rows) = terminal::size()?;
                            execute!(stdout, cursor::MoveTo(0, rows.saturating_sub(2)))?; 
                            println!("Refreshing app...");
                            // Reinitialize app to refresh
                            app = App::new().await?; 
                            // clear the line above when refresh is finished.
                            let _ = move_and_clear_line(&mut stdout, 2);
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

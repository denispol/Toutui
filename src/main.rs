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
use crate::db::crud::*;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem , ListState,  Paragraph, StatefulWidget,
        Widget, Wrap
    },
};
use ratatui::Terminal;
use crossterm::terminal::{self, size};
use crate::player::integrated::progression_stats::*;


// test for automatic refresh
pub fn render_player(area: Rect, buf: &mut ratatui::buffer::Buffer, message: &str) {
    let block_width = area.width / 3;
    let block_x = (area.width - block_width) / 2;
    let block_area = Rect::new(block_x, area.y, block_width, area.height);
    let block = Block::default();

    // 1/3 block
    let left_block_width = block_width - (block_width * 2) / 3;
    let left_block_area = Rect::new(block_x, area.y, left_block_width, area.height);

    // 2/3 block
    let text_area_width = (block_width * 2) / 3;
    let text_area = Rect::new(block_x + left_block_width, area.y, text_area_width, area.height);

    // Text area for 2/3 block
    let paragraph = Paragraph::new(message)
        .centered()
        .block(Block::default());

    // Image 1/3 block
    let left_block = Block::default()
        .style(Style::default().bg(Color::Gray));

    // Render
    paragraph.render(text_area, buf); // Right text area (2/3 block)
    left_block.render(left_block_area, buf); // Image area (1/3 block)
    block.render(block_area, buf); // General block
}

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

        // init current username
        let mut username: String = String::new();
        if let Some(var_username) = database.default_usr.get(0) {
            username = var_username.clone();
        }
        // init is_vlc_launched_first_time 
        let _ = update_is_vlc_launched_first_time("1", username.as_str());
        let value = get_is_vlc_launched_first_time(username.as_str());
        info!("[main][is_vlc_launched_first_time] {}", value);

        let mut app = App::new().await?;
        let mut terminal = ratatui::init();
        //let mut terminal2 = ratatui::init();

        // Running the app in a loop
        loop {
            // If `app` variable is reinitialized below (`app = App::new().await?`), it will be taken into account and data will be refreshed
            // Otherwise, the current `app` variable will still be used.
            let result = app.run(&mut terminal);

            if let Err(e) = result {
                eprintln!("Error running the app: {:?}", e);
                error!("Error running the app: {:?}", e);
            }

            //  
            terminal.draw(|frame| {
            //terminal2.draw(|frame| {
                let (term_width, term_height) = terminal::size().unwrap();
                let width = 200;
                let height = 2;
                let x = (term_width.saturating_sub(width)) / 2;
                let y = (term_height.saturating_sub(height)) / 2;
                let area = Rect::new(x, y, width, height);
                let message = get_dynamic_text();

                let mut buf = frame.buffer_mut();
                render_player(area, &mut buf, message.as_str()); 
            })?;


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

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


#[tokio::main]
async fn main() -> Result<()> {
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
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    crossterm::event::KeyCode::Char('r') => {
                       // println!("Refreshing app...");
                        app = App::new().await?; 
                    }
                    crossterm::event::KeyCode::Char('q') => {
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

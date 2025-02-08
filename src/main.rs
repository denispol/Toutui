mod app;
mod config;
mod api;
mod ui;
mod player;
mod logic;
mod db;
use app::App;
use color_eyre::Result;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

#[tokio::main]
async fn main() -> Result<()> {

    // Init app
    color_eyre::install()?;
    let mut app = App::new().await?;
    let mut terminal = ratatui::init();

    loop {
        // If `app` variable is reinit bellow (`app = App::new().await?; `), it will be take into account, and data will be refresh
        // else, current `app` variable still are still used.
        let result = app.run(&mut terminal);

        if let Err(e) = result {
            eprintln!("Error running the app: {:?}", e);
        }

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('R') => {
                       // println!("Refreshing app...");
                        
                        app = App::new().await?; // reinit for refresh
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

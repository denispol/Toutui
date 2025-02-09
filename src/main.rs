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
    color_eyre::install()?;
    let app = App::new().await?;
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result

}

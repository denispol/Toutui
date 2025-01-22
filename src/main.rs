mod app;
mod config;
mod api;
mod ui;

use app::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;  // Handling erros

    // Init app (connection and data fetching)
    let application = App::new().await?;
    
    // Run UI
    application.run_ui()?;

    Ok(())
}


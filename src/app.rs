use crate::api::auth::login;
use crate::api::libraries::get_continue_listening;
use crate::config::load_config;
use crate::ui::tui::run;
use color_eyre::Result;
use ratatui::init;
use color_eyre::eyre::Report;

pub struct App {
    pub token: Option<String>,
    pub titles: Vec<String>,
}

impl App {
    /// Init app
    pub async fn new() -> Result<Self, Report> {
        // Load config file from config.toml
        let config = load_config()?;
        
        // API Auth
        let token = login(&config.credentials.id.to_string(), &config.credentials.password.to_string()).await?;

        // Retrieve "Continue Listening"
        let titles = get_continue_listening(&token).await?;

        Ok(Self {
            token: Some(token.to_string()),
            titles,
        })
    }

    /// Main fun to start UI app
    pub fn run_ui(&self) -> Result<()> {
        let terminal = init();
        let result = run(terminal, &self.titles.join("\n"));
        ratatui::restore();
        result
    }
}


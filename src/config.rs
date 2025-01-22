use config::{Config as ConfigLib, File};
use serde::Deserialize;
use color_eyre::eyre::{Result, Report};

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub credentials: Credentials,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub id: String,
    pub password: String,
}

/// Fun to load config from `config.toml` file
pub fn load_config() -> Result<ConfigFile> {
    // Build config from a file
    let config = ConfigLib::builder()
        .add_source(File::with_name("../config.toml"))
        .build()
        .map_err(|e| Report::new(e))?;

    // Extract infos `credentials` in the config 
    let credentials: Credentials = config.get("credentials")
        .map_err(|e| Report::new(e))?;

    // Return config struct
    Ok(ConfigFile { credentials })
}


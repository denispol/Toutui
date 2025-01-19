use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::header::AUTHORIZATION;
use config::{Config, File};

// config file
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub credentials: Credentials,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub id: String,
    pub password: String,
}


// load the config file
fn load_config() -> Result<ConfigFile, Box<dyn Error>> {
    let config = Config::builder()
        .add_source(File::with_name("../config.toml"))
        .build()?;

    let credentials: Credentials = config.get("credentials")?;

    Ok(ConfigFile { credentials })
}


// 
#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    user: UserInfo,
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    token: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?; // load config file
    let login_url = "https://audiobook.nuagemagique.duckdns.org/login";

    let client = Client::new();

    let login_data = LoginRequest {
        username: config.credentials.id.to_string(),
        password: config.credentials.password.to_string(),
    };

    let response = client
        .post(login_url)
        .header("Content-Type", "application/json") 
        .json(&login_data) 
        .send()
        .await?;

    // Vérifie le succès de la requête
    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await?;
        println!("⚡Connexion réussie !");
        println!("Token récupéré : {}", login_response.user.token);
        let get_all_libraries = client
            .get("https://audiobook.nuagemagique.duckdns.org/api/libraries")
            .header(AUTHORIZATION, format!("Bearer {}", &login_response.user.token))
            .send()
            .await?;

        let body = get_all_libraries.text().await?;

        let get_library_item = client
            .get("https://audiobook.nuagemagique.duckdns.org/api/items/da63a1c0-973d-4d55-861c-179e70806232")
            .header(AUTHORIZATION, format!("Bearer {}", &login_response.user.token))
            .send()
            .await?;
        let body2 = get_library_item.text().await?;


    println!("Réponse brute : {}", body2);
    } else {
        eprintln!(
            "Erreur de connexion : {} ({}).",
            response.status(),
            response.text().await?
        );
    }


    Ok(())
}



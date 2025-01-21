use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::header::AUTHORIZATION;
use config::{Config, File};
use serde_json::Value;
use std::process::Command;
use std::process::Output;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

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

#[derive(Deserialize, Debug)]
struct PlayResponse {
    id: String,
    userId: String,
    libraryId: String,
    libraryItemId: String,
    episodeId: String,
    mediaType: String,
    displayTitle: String,
    displayAuthor: String,
    coverPath: String,
    duration: f64,
    playMethod: u32,
    mediaPlayer: String,
    serverVersion: String,
    date: String,
    dayOfWeek: String,
    timeListening: u32,
    startTime: u64,
    currentTime: u64,
    startedAt: u64,
    updatedAt: u64,
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
        //let get_all_libraries = client
        //    .get("https://audiobook.nuagemagique.duckdns.org/api/libraries")
        //    .header(AUTHORIZATION, format!("Bearer {}", &login_response.user.token))
        //    .send()
        //    .await?;

        //let body = get_all_libraries.text().await?;

        let get_library = client
            .get("https://audiobook.nuagemagique.duckdns.org/api/libraries/64c39f84-9c58-4045-a89c-e17a6d990768/personalized")
            .header(AUTHORIZATION, format!("Bearer {}", &login_response.user.token))
            .send()
            .await?;
        let body2 = get_library.text().await?;
        let parsed_json: Value = serde_json::from_str(&body2)?;
        //println!("{}", serde_json::to_string_pretty(&parsed_json)?);
        if let Some(items) = parsed_json.as_array() {
            for item in items {
                if item["label"] == "Continue Listening" {
                    if let Some(entities) = item["entities"].as_array() {
                        for entity in entities {
                            if let Some(title) = entity["media"]["metadata"]["title"].as_str() {
                                println!("{}", title);
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &title);
    ratatui::restore();
    result.expect("REASON")
                            }
                        }
                    }
                }
            }
        }
//        //play //
//        let play = client
//            .post("https://audiobook.nuagemagique.duckdns.org/api/items/a3c6a644-b77b-4737-a70e-88d4def19d6c/play")
//            .header(AUTHORIZATION, format!("Bearer {}", &login_response.user.token))
//            .header("Content-Type", "application/json") 
//            // .json(&login_data) 
//            .send()
//            .await?;
//
//        let output: Output = Command::new("vlc")
//            .arg("https://audiobook.nuagemagique.duckdns.org/hls/6bc1d51a-dc06-438e-8229-eb7b9311fe06/output.m3u8?token=")
//            .output()
//            .expect("Failed to execute program");
//
//        //Ok(output)

//        let raw_response = play.text().await?;
//        println!("Réponse brute: {}", raw_response);

//        if play.status().is_success() {
//            let play_response: PlayResponse = play.json().await?;
//            println!("Token récupéré : {}", play_response.dayOfWeek);}
//        else {
//        eprintln!(
//            "Erreur de connexion : {} ({}).",
//            play.status(),
//            play.text().await?
//        );
//
//    }} else {
//        eprintln!(
//            "Erreur de connexion : {} ({}).",
//            response.status(),
//            response.text().await?
//        );
    }


    Ok(())
}

fn run(mut terminal: DefaultTerminal, message: &str) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, message))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, message: &str) {
    frame.render_widget(message, frame.area());
}

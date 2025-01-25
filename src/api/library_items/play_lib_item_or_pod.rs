use reqwest::Client; 
use serde::{Deserialize, Serialize}; 
use std::error::Error; 
use color_eyre::eyre::{Result, Report}; // Use Report rather than Box<dyn Error> 
use std::process::Command;
use std::process::Output;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use serde_json::json;


/// Play a Library Item or Podcast Episode
/// This endpoint starts a playback session for a library item or podcast episode.
/// https://api.audiobookshelf.org/#play-a-library-item-or-podcast-episode


pub async fn post_start_playback_session(token: Option<String>, id_library_items: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();

    let params = json!({
        "forceDirectPlay": true, // avoid latency load, allow view chapter, cover etc.(the .m3u8 stream the original format, ex: .m4b) when playing with vlc
        "mediaPlayer": "vlc",
    });

    let response = client
        .post(format!(
            "https://audiobook.nuagemagique.duckdns.org/api/items/{}/play", 
            id_library_items
        ))
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token.as_deref().unwrap_or("")))
        .json(&params)
        .send()
        .await?;

    // Récupérer la réponse JSON
    let v: Value = response.json().await?;

    let current_time = v["currentTime"]
        .as_f64()
        .unwrap_or(0.0);
    let content_url = v["audioTracks"][0]["contentUrl"]
        .as_str()
        .unwrap_or("");

    let data_for_vlc = vec![current_time.to_string(), content_url.to_string()];
    println!("{:?}", data_for_vlc);

    Ok(data_for_vlc)
}

pub async fn start_vlc(current_time: &String, content_url: &String, token: &str) -> Output {
    let output: Output = Command::new("vlc")
        .arg(format!("--start-time={}", current_time))
        .arg(format!("https://audiobook.nuagemagique.duckdns.org{}?token={}", content_url, token))
        .output()
        .expect("Failed to execute program");
    println!("{}", current_time);
    output
}


use reqwest::Client; 
use color_eyre::eyre::{Result}; 
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use serde_json::json;
use crate::player::vlc::fetch_vlc_data::get_vlc_version;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Play a Library Item or Podcast Episode
/// This endpoint starts a playback session for a library item or podcast episode.
/// https://api.audiobookshelf.org/#play-a-library-item-or-podcast-episode

// play book 
pub async fn post_start_playback_session_book(token: Option<&String>, id_library_item: &str) -> Result<Vec<String>, reqwest::Error> {
    let mut vlc_version = String::new();
    match get_vlc_version().await {
        Ok(version) => {vlc_version = version;}
        Err(e) => {
            //eprintln!("{}", e),
        }
    }
    println!("{:?}", vlc_version);
    let client = Client::new();

    let params = json!({
        "forceDirectPlay": true, // avoid latency load, allow view chapter, cover etc.(the .m3u8 stream the original format, ex: .m4b) when playing with vlc
        "mediaPlayer": format!("VLC v{}", vlc_version),
        "deviceInfo": {  
            "clientName": "Toutui",
            "clientVersion": format!("v{}", VERSION),
    }});

    let response = client
        .post(format!(
            "https://audiobook.nuagemagique.duckdns.org/api/items/{}/play", 
            id_library_item
        ))
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .json(&params)
        .send()
        .await?;

    // Retrieve JSON response
    let v: Value = response.json().await?;

    // Retrieve data
    let id_session = v["id"]
        .as_str()
        .unwrap_or("");
    let current_time = v["currentTime"]
        .as_f64()
        .unwrap_or(0.0);
    let content_url = v["audioTracks"][0]["contentUrl"]
        .as_str()
        .unwrap_or("");
    let duration = v["audioTracks"][0]["duration"]
        .as_f64()
        .unwrap_or(0.0);
    let duration: u32 = duration as u32;

    let info_item = vec![current_time.to_string(), content_url.to_string(), duration.to_string(), id_session.to_string()];

    Ok(info_item)
}
// play podcast episode
pub async fn post_start_playback_session_pod(token: Option<&String>, id_library_item: &str, pod_ep_id: &str) -> Result<Vec<String>, reqwest::Error> {
    let mut vlc_version = String::new();
    match get_vlc_version().await {
        Ok(version) => {vlc_version = version;}
        Err(e) => {
            //eprintln!("{}", e),
        }
    }
    let client = Client::new();

    let params = json!({
        "forceDirectPlay": true, // avoid latency load, allow view chapter, cover etc.(the .m3u8 stream the original format, ex: .m4b) when playing with vlc
        "mediaPlayer": format!("VLC v{}", vlc_version),
        "deviceInfo": {  
            "clientName": "Toutui",
            "clientVersion": format!("v{}", VERSION),
    }});

    let response = client
        .post(format!(
            "https://audiobook.nuagemagique.duckdns.org/api/items/{}/play/{}", 
            id_library_item, pod_ep_id))
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .json(&params)
        .send()
        .await?;

    // Retrieve JSON response
    let v: Value = response.json().await?;

    // Retrieve data
    let id_session = v["id"]
        .as_str()
        .unwrap_or("");
    let current_time = v["currentTime"]
        .as_f64()
        .unwrap_or(0.0);
    let content_url = v["audioTracks"][0]["contentUrl"]
        .as_str()
        .unwrap_or("");
    let duration = v["audioTracks"][0]["duration"]
        .as_f64()
        .unwrap_or(0.0);
    let duration: u32 = duration as u32;

    let info_item = vec![current_time.to_string(), content_url.to_string(), duration.to_string(), id_session.to_string()];

    Ok(info_item)
}

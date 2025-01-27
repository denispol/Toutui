use reqwest::Client; 
use color_eyre::eyre::{Result}; 
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use serde_json::json;


/// Play a Library Item or Podcast Episode
/// This endpoint starts a playback session for a library item or podcast episode.
/// https://api.audiobookshelf.org/#play-a-library-item-or-podcast-episode

pub async fn post_start_playback_session(token: Option<&String>, id_library_item: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();

    let params = json!({
        "forceDirectPlay": true, // avoid latency load, allow view chapter, cover etc.(the .m3u8 stream the original format, ex: .m4b) when playing with vlc
        "mediaPlayer": "vlc",
    });

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
    let current_time = v["currentTime"]
        .as_f64()
        .unwrap_or(0.0);
    let content_url = v["audioTracks"][0]["contentUrl"]
        .as_str()
        .unwrap_or("");

    let data_for_vlc = vec![current_time.to_string(), content_url.to_string()];

    Ok(data_for_vlc)
}

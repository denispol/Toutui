use reqwest::Client; 
use serde::{Deserialize, Serialize}; 
use std::error::Error; 
use color_eyre::eyre::{Result, Report}; // Use Report rather than Box<dyn Error> 
use std::process::Command;
use std::process::Output;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;


/// Play a Library Item or Podcast Episode
/// This endpoint starts a playback session for a library item or podcast episode.
/// https://api.audiobookshelf.org/#play-a-library-item-or-podcast-episode
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub library_id: Option<String>,
    pub library_item_id: Option<String>,
    pub episode_id: Option<String>,
    pub media_type: Option<String>,
    pub media_metadata: Option<MediaMetadata>,
    pub chapters: Option<Vec<Value>>,
    pub display_title: Option<String>,
    pub display_author: Option<String>,
    pub cover_path: Option<String>,
    pub duration: Option<f64>,
    pub play_method: Option<i64>,
    pub media_player: Option<String>,
    pub device_info: Option<DeviceInfo>,
    pub date: Option<String>,
    pub day_of_week: Option<String>,
    pub time_listening: Option<i64>,
    pub start_time: Option<i64>,
    pub current_time: Option<i64>,
    pub started_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub audio_tracks: Option<Vec<AudioTrack>>,
    pub video_track: Option<Value>,
    pub library_item: Option<LibraryItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<String>,
    pub genres: Option<Vec<String>>,
    pub feed_url: Option<String>,
    pub image_url: Option<String>,
    pub itunes_page_url: Option<String>,
    pub itunes_id: Option<i64>,
    pub itunes_artist_id: Option<i64>,
    pub explicit: Option<bool>,
    pub language: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub ip_address: Option<String>,
    pub client_version: Option<String>,
    pub server_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub index: Option<i64>,
    pub start_offset: Option<i64>,
    pub duration: Option<f64>,
    pub title: Option<String>,
    pub content_url: Option<String>,
    pub mime_type: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub filename: Option<String>,
    pub ext: Option<String>,
    pub path: Option<String>,
    pub rel_path: Option<String>,
    pub size: Option<i64>,
    pub mtime_ms: Option<i64>,
    pub ctime_ms: Option<i64>,
    pub birthtime_ms: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    pub id: Option<String>,
    pub ino: Option<String>,
    pub library_id: Option<String>,
    pub folder_id: Option<String>,
    pub path: Option<String>,
    pub rel_path: Option<String>,
    pub is_file: Option<bool>,
    pub mtime_ms: Option<i64>,
    pub ctime_ms: Option<i64>,
    pub birthtime_ms: Option<i64>,
    pub added_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub last_scan: Option<i64>,
    pub scan_version: Option<String>,
    pub is_missing: Option<bool>,
    pub is_invalid: Option<bool>,
    pub media_type: Option<String>,
    pub media: Option<Media>,
    pub size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub library_item_id: Option<String>,
    pub cover_path: Option<String>,
    pub tags: Option<Vec<Value>>,
    pub auto_download_episodes: Option<bool>,
    pub auto_download_schedule: Option<String>,
    pub last_episode_check: Option<i64>,
    pub max_episodes_to_keep: Option<i64>,
    pub max_new_episodes_to_download: Option<i64>,
    pub size: Option<i64>,
}

pub async fn post_start_playback_session(token: Option<String>, id_library_items: &str) -> Result<Vec<Root>>{
            let client = Client::new();

            let response = client
            .post(format!("https://audiobook.nuagemagique.duckdns.org/api/items/{}/play", id_library_items))
            .header(AUTHORIZATION, format!("Bearer {}", token.as_deref().unwrap_or("")))
            .header("Content-Type", "application/json")
            // .json(&login_data)
            .send()
            .await?;

        let play_library_items: Vec<Root> = response.json().await?;
        println!("{:?}", play_library_items);
        Ok(play_library_items)
}

pub async fn start_vlc(content_url: &str, token: Option<String>) -> Result<Output, Box<dyn Error>> {
    let output: Output = Command::new("vlc")
        .arg(format!("https://audiobook.nuagemagique.duckdns.org{}?token={}", content_url, token.as_deref().unwrap_or("")))
        .output()
        .expect("Failed to execute program");
    Ok(output)
}


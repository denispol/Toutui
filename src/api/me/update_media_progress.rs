use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;

/// Create/Update Media Progress
/// This endpoint creates/updates your media progress for a library item or podcast episode.
/// https://api.audiobookshelf.org/#create-update-media-progress

pub async fn update_media_progress(id_library_item: &str, token: Option<&String>, current_time: Option<u32>, duration: &String) -> Result<(), Box<dyn Error>> {

    // Build client reqwest
    let client = reqwest::Client::new();

    // convert data before init progress (float)
    let duration_f32 = duration.parse::<f32>().unwrap();
    let current_time_f32: f32 = current_time.unwrap() as f32;

    // init  progress
    let progress = current_time_f32 / duration_f32 ;

    // json bosy
    let body = json!({
        "progress" : progress,
        "currentTime": current_time,
    });

    // Patch request
    let response = client
        .patch(format!(
                "https://audiobook.nuagemagique.duckdns.org/api/me/progress/{}", 
                id_library_item
        ))
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await?;

    // 
    //let status = response.status();
    //let response_text = response.text().await?;

   // println!("Statut: {}", status);
   // println!("Réponse: {}", response_text);

    Ok(())
}

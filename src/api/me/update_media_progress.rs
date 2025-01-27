use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;

/// Create/Update Media Progress
/// This endpoint creates/updates your media progress for a library item or podcast episode.
/// https://api.audiobookshelf.org/#create-update-media-progress

#[allow(dead_code)]
pub async fn update_media_progress(id_library_item: &str, token: Option<&String>, current_time: Option<u32> ) -> Result<(), Box<dyn Error>> {

    // Construire le client reqwest
    let client = reqwest::Client::new();

    // Corps de la requête en JSON
    let body = json!({
        //"progress" : 0.75,
        "currentTime": current_time,
    });

    // Effectuer la requête PATCH
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

    // Lire et afficher la réponse
    let status = response.status();
    let response_text = response.text().await?;

    println!("Statut: {}", status);
    println!("Réponse: {}", response_text);

    Ok(())
}

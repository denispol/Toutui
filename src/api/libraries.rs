use reqwest::Client;
use serde_json::Value;
use reqwest::header::AUTHORIZATION;
use color_eyre::eyre::{Result, Report};

/// Get a Library's Personalized View
/// https://api.audiobookshelf.org/#get-a-library-39-s-personalized-view

/// filter only book continue to listening from personalized view
pub async fn get_continue_listening(token: &str) -> Result<Vec<String>> {
    let client = Client::new();
    let url = "https://audiobook.nuagemagique.duckdns.org/api/libraries/64c39f84-9c58-4045-a89c-e17a6d990768/personalized";

    // Send GET
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    // Check response's status
    if !response.status().is_success() {
        return Err(Report::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch data from the API",
        )));
    }

    let body: Value = response.json().await?;

    let mut titles = Vec::new();

    // Filter titles
    if let Some(items) = body.as_array() {
        for item in items {
            if item["label"] == "Continue Listening" {
                if let Some(entities) = item["entities"].as_array() {
                    for entity in entities {
                        if let Some(title) = entity["media"]["metadata"]["title"].as_str() {
                            titles.push(title.to_string());
                        }
                    }
                }
            }
        }
    }

    // Return books
    Ok(titles)
}


use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;

pub async fn update_media_progress() -> Result<(), Box<dyn Error>> {
    let url = "https://audiobook.nuagemagique.duckdns.org/api/me/progress/a3c6a644-b77b-4737-a70e-88d4def19d6c";
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI3NTk2MmQzMy05YmNmLTQyYzYtODY2ZC0yODcwYjQzYWM2MWYiLCJ1c2VybmFtZSI6ImFsYmFuIiwiaWF0IjoxNzMyNDUyMTEwfQ.VCiv72-0PxLhRdJen3KKi8BE_QDPBGmKQCNOzHf25lQ";

    // Construire le client reqwest
    let client = reqwest::Client::new();

    // Corps de la requête en JSON
    let body = json!({
        "progress" : 0.55,
        "currentTime": 1000
    });

    // Effectuer la requête PATCH
    let response = client
        .patch(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
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

use reqwest::Client; 
use color_eyre::eyre::{Result}; 
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use serde_json::json;

/// This endpoint closes an open listening session. Optionally provide sync data to update the session before closing it.
/// https://api.audiobookshelf.org/#close-an-open-session

// close an open session
pub async fn close_session(token: Option<&String>, session_id: &str, current_time: Option<u32>, time_listened: u64, server_address: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let params = json!({
        "currentTime": format!("{}", current_time.unwrap_or(0)), 
        "timeListened": format!("{}", time_listened),
    });

    let response = client
        .post(format!(
            "{}/api/session/{}/close", 
            server_address,
            session_id
        ))
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .json(&params)
        .send()
        .await?;

    Ok(())
}



pub async fn close_session_without_send_prg_data(token: Option<&String>, session_id: &str, server_address: String) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client
        .post(format!(
            "{}/api/session/{}/close", 
            server_address,
            session_id
        ))
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .send()
        .await?;

    Ok(())
}

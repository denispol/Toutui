use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use color_eyre::eyre::{Result, Report}; // Uuse Report rather than Box<dyn Error>

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

/// LOGIN
/// https://api.audiobookshelf.org/#server

/// The login function takes a username and password, makes a POST request and returns a token.
pub async fn login(username: &str, password: &str) -> Result<String> {
    let login_url = "https://audiobook.nuagemagique.duckdns.org/login";
    let client = Client::new();

    // Struct for data request
    let login_data = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    // Send POST request
    let response = client
        .post(login_url)
        .header("Content-Type", "application/json")
        .json(&login_data)
        .send()
        .await?;

    // Checking the status of the response
    if response.status().is_success() {
        let login_response: LoginResponse = response.json().await?;
        Ok(login_response.user.token) // Return user token
    } else {
        Err(Report::new(std::io::Error::new(std::io::ErrorKind::Other, "Login failed"))) 
    }
}


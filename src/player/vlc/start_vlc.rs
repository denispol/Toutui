use std::process::Command;
use std::process::Output;

pub async fn start_vlc(current_time: &String, port: &str, content_url: &String, token: Option<&String>, title: String, subtitle: String, author: String) -> Output {
    let output: Output = Command::new("vlc")
        .arg(format!("--start-time={}", current_time))
        .arg("--extraintf")
        .arg("rc")
        .arg("--rc-host")
        .arg(format!("localhost:{}", port))
        .arg(format!("https://audiobook.nuagemagique.duckdns.org{}?token={}", content_url, token.unwrap()))
        .arg("--meta-description")
        .arg(author)
        .arg("--meta-title")
        .arg(subtitle)
        .arg("--meta-artist")
        .arg(title)
        .output()
        .expect("Failed to execute program");

    output
}

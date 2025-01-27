use std::process::Command;
use std::process::Output;

pub async fn start_vlc(current_time: &String, port: &str, content_url: &String, token: Option<&String>) -> Output {
    let output: Output = Command::new("vlc")
        .arg(format!("--start-time={}", current_time))
        .arg("--extraintf")
        .arg("rc")
        .arg("--rc-host")
        .arg(format!("localhost:{}", port))
        .arg(format!("https://audiobook.nuagemagique.duckdns.org{}?token={}", content_url, token.unwrap()))
        .output()
        .expect("Failed to execute program");

    output
}

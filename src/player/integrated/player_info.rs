use std::net::TcpStream;
use std::io::{self, Write, Read};
use log::info;


pub fn player_info(address: &str, port: &str) -> Vec<String> {

    let mut player_info = Vec::new();
    let current_time = chrono::Local::now().to_rfc3339();

    match get_info(address, port) {
        Ok(title) => player_info.push(title),
        Err(e) => player_info.push(format!("Error: {}", e)),
    }
    player_info.push(current_time);

    info!("{:?}", player_info);
    player_info
}


pub fn get_info(address: &str, port: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;
    writeln!(stream, "get_time")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Nettoyage de la réponse
    let title = response
        .lines() // Découpe en lignes
        .map(|line| line.trim()) // Supprime les espaces et caractères spéciaux
        .filter(|line| !line.is_empty()) // Ignore les lignes vides
        .filter(|line| !line.starts_with("VLC media player")) // Ignore l'en-tête VLC
        .filter(|line| !line.contains("Command Line Interface initialized")) // Ignore le message CLI
        .filter(|line| !line.contains("> Shutting down.")) // Ignore le message de fermeture
        .find(|line| line.starts_with(">")) // Prend la première ligne qui commence par ">"
        .map(|line| line.trim_start_matches("> ").to_string()) // Supprime le "> " au début
        .unwrap_or_else(|| "Unknown title".to_string()); // Valeur par défaut

    Ok(title)
}


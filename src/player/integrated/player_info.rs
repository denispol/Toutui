use log::info;
use crate::db::crud::*;

pub fn player_info(username: &str) -> Vec<String> {
    let mut player_info = Vec::new();

    match get_listening_session() {
        Ok(Some(session)) => {
            player_info.push(session.title);
            player_info.push(session.author);
            player_info.push(session.current_time.to_string());
            player_info.push(session.duration);
            player_info.push(session.elapsed_time.to_string());
        }
        Ok(None) => {
            player_info.push(format!("N/A"));
        }
        Err(e) => {
            player_info.push(format!("Error"));
            info!("[player_info] Error retrieving data: {}", e);
        }
    }

    player_info.push(get_speed_rate(username).to_string());

    player_info
}

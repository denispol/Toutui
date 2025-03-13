use std::io::{self, Write};
use std::net::TcpStream;
use crate::db::crud::*;

pub fn handle_key_player(key: &str, address: &str, port: &str, is_playback: &mut bool, username: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    let jump_forward = "10";

    match key {
        // toggle playback/pause
        " " => {
            if *is_playback {
                writeln!(stream, "pause")?;
            } else {
                writeln!(stream, "play")?;
            }
            *is_playback = !*is_playback;
        }
        // jump forward
        "p" => {
            writeln!(stream, "seek +{}", jump_forward)?;
        }
        // jump backward
        "u" => {
            writeln!(stream, "seek -{}", jump_forward)?;
        }
        // next chapter
        "P" => {
            writeln!(stream, "chapter_n")?;
        }
        // previous chapter
        "U" => {
            writeln!(stream, "chapter_p")?;
        }
        // volume up
        "o" => {
            writeln!(stream, "volup")?;
        }
        // volume down
        "i" => {
            writeln!(stream, "voldown")?;
        }
        // speed rate up
        "O" => {
            let _ = update_speed_rate(username, true);
            let speed_rate = get_speed_rate(username);
            writeln!(stream, "rate {}", speed_rate)?;
        }
        // speed rate up
        "I" => {
            let _ = update_speed_rate(username, false);
            let speed_rate = get_speed_rate(username);
            writeln!(stream, "rate {}", speed_rate)?;
        }
        // shutdown
        "Y" => {
            writeln!(stream, "shutdown")?;
        }
        _ => {}
    }

    Ok(())
}


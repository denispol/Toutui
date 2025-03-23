use std::io::{self, Write};
use std::net::TcpStream;
use crate::db::crud::*;


pub fn handle_key_player(key: &str, address: &str, port: &str, is_playback: &mut bool, username: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    let jump = "10";

    match key {
        // toggle playback/pause
        " " => {
            match get_listening_session() {
                Ok(Some(session)) => {
                    if session.is_playback {
                    let _ = update_is_playback("0", session.id_session.as_str());
                    } else {
                    let _ = update_is_playback("1", session.id_session.as_str());
                    }
                }
                Ok(None) => {

                }
                Err(_e) => {

                }
            }
            if *is_playback {
                writeln!(stream, "pause")?;
            } else {
                writeln!(stream, "play")?;
            }
            *is_playback = !*is_playback;
        }
        // jump forward
        "p" => {
            writeln!(stream, "seek +{}", jump)?;
        }
        // jump backward
        "u" => {
            writeln!(stream, "seek -{}", jump)?;
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


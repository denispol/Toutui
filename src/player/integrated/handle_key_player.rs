use std::io::{self, Write};
use std::net::TcpStream;

pub fn handle_key_player(key: &str, address: &str, port: &str, is_playback: &mut bool) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    match key {
        " " => {
            if *is_playback {
                writeln!(stream, "pause")?;
            } else {
                writeln!(stream, "play")?;
            }
            *is_playback = !*is_playback;
        }
        _ => {}
    }

    Ok(())
}


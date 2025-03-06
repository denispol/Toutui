use std::net::TcpStream;
use std::io::{self, Write};
use log::info;

pub fn quit_vlc(address: &str, port: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port))?;

    writeln!(stream, "shutdown")?;

    info!("[quit_vlc.rs] VLC successfully quit");

    Ok(())
}

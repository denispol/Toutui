use std::fs::OpenOptions;
use std::io::{self, Write};
use vlc_rc::Client;
use tokio::net::TcpStream;


/// TODO : PUT ALL PRINT IN LOG ///
/// This function : 
/// allow to connect and remotely ctrl VLC (with vlc-rc crate) on the port that was provided (Client::connect(format!("localhost:{}", port))) 
/// if connection is successul, fecth data thanks to remotly control
/// this fn is in the loop and run while vlc is running (bu checking if the port is still open)
pub async fn fetch_vlc_data(port: &str) -> Result<Option<u32>, io::Error> {
    loop {
        // Check is VLC is running. If it's not the case, loop is break
        // and CLient connect to vlc and fetchind seconds will not be exectued
        if !is_vlc_running("1234").await {
            //println!("VLC is not running, exiting fetch loop.");
            break Ok(None); // Exit loop (fetching data) if VLC is not running anymore
        }

        // vlc-rc crafte connect to VLC to be able to fetch data 
        let mut player = match Client::connect(format!("localhost:{}", port)) {
            Ok(player) => player,
            Err(e) => {
                if let Err(file_error) = log_error_to_file(&e.to_string()) {
                    eprintln!("Failed to log error: {}", file_error);
                }
                continue;
            }
        };
        
    // Fetch VLC current time (if connection is successful)
        let seconds = match player.get_time() {
            Ok(Some(value)) => Some(value),
            Ok(None) => None,
            Err(e) => {
                eprintln!("Failed to fetch time from VLC: {}", e);
                None
            }
        };

        // Print and return the fetched seconds
        if let Some(sec) = seconds {
            return Ok(Some(sec)); // Return seconds once fetched
        }

        // Sleep to fetch data every second and avoid CPU overload
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

}

// check if VLC is running by checking if the port used by the app to open VLC is open
pub async fn is_vlc_running(port: &str) -> bool {
    match TcpStream::connect(format!("localhost:{}", port)).await {
        Ok(_) => {
            println!("VLC is still running (port {} is open).", port);
            true
        }
        Err(_) => {
            //println!("VLC is not running (port {} is closed).", port);
            false
        }
    }
}


fn log_error_to_file(error_message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("vlc_errors.txt")?;
    writeln!(file, "{}", error_message)?;
    Ok(())
}
#[allow(dead_code)]
fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}



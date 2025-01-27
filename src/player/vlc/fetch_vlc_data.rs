use std::fs::OpenOptions;
use std::io::{self, Write};
use vlc_rc::Client;
use tokio::net::TcpStream;


/// TODO : PUT ALL PRINTLN IN LOG ///
/// This function : 
/// allow to connect and remotely ctrl VLC (with vlc-rc crate) on the port that was provided (Client::connect(format!("localhost:{}", port))) 
/// if connection is successul, fecth data thanks to remotly control
/// this fn is in the loop and run while vlc is running (bu checking if the port is still open)
pub async fn fetch_vlc_data(port: &str) {
    loop {
        // Check is VLC is running. If it's not the case, loop is break
        // and CLient connect to vlc and fetchind seconds will not be exectued
        if !is_vlc_running("1234").await {
            //println!("VLC is not running, exiting fetch loop.");
            break; // Exit loop (fetching data) if VLC is not running anymore
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
        // Fetch VLC current time (if connection to is successful)
        let seconds = match player.get_time() {
            Ok(Some(value)) => value.to_string(),
            Ok(None) => "N/A".to_string(),
            Err(e) => {
                if let Err(file_error) = log_error_to_file(&format!("Error fetching time: {}", e)) {
                    eprintln!("Failed to log error: {}", file_error);
                }
                eprintln!("Failed to fetch time from VLC: {}", e);

                // default value (TODO: remove this part )
                "Error".to_string()
            }
        };

        // Print in the file 
        println!("{}", seconds);
        if let Err(e) = write_to_file("vlc_time_log.txt", &seconds) {
            eprintln!("Failed to write to file: {}", e);
        }

        // allow to fetch data every seconds and avoid CPU overload
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

}

// check is VLC is running by checking if the port used by the app to open VLC is open
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

fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}



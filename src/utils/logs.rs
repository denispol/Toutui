use log::LevelFilter;
use fern::Dispatch;
use chrono::Local;
use std::fs::OpenOptions;
use std::env;

pub fn setup_logs() -> Result<(), fern::InitError> {

let config_dir = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
    let mut path = dirs::home_dir().expect("Unable to find the user's home directory");
    
    // Check for macOS, then use the appropriate directory
    if cfg!(target_os = "macos") {
        path.push("Library/Preferences");
    } else {
        path.push(".config"); // For other OS like Linux, use ~/.config
    }

    path // Return the constructed path as a PathBuf
});

// Construct the log file path by appending the "toutui/toutui.log" file
let log_path = config_dir.join("toutui/toutui.log");

    // Create or append into the file
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_path) // path and name
        .unwrap();

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                    "{} [{}] - {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    message
            ))
        })
    .level(LevelFilter::Info) 
        .chain(log_file) // redirect logs to the file 
        .apply()?; 

    Ok(())
}

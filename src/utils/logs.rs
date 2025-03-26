use log::LevelFilter;
use fern::Dispatch;
use chrono::Local;
use std::fs::OpenOptions;

pub fn setup_logs() -> Result<(), fern::InitError> {
    let mut log_path = if cfg!(target_os = "macos") {
    let mut path = dirs::home_dir().expect("Unable to find the user's home directory");
    path.push("Library/Application Support");
    path
} else {
    dirs::config_dir().expect("Unable to find the .config directory")
};
log_path.push("toutui/toutui.log");

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

use log::LevelFilter;
use fern::Dispatch;
use chrono::Local;
use std::fs::OpenOptions;
use std::path::PathBuf;


pub fn setup_logs() -> Result<(), fern::InitError> {
    let mut log_path = if cfg!(target_os = "macos") {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".config");
    path
} else {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
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

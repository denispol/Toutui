use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use std::io::{self, Write};
use std::process;

// exit the app
pub fn clean_exit() {
    let _ = disable_raw_mode(); 
    let mut stdout = io::stdout();
    let _ = crossterm::execute!(stdout, LeaveAlternateScreen); 
    let _ = stdout.flush(); 
    process::exit(0);
}

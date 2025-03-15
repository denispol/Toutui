use std::io::{Result, Stdout};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor},
    terminal, cursor,
};
use crate::config::*;

// pop up message
pub fn player_progress(stdout: &mut Stdout, lines_from_bottom: u16, message: &str) -> Result<()> {
    // import backgorund color
    let mut color = Vec::new();
    if let Ok(cfg) = load_config() {
        color = cfg.colors.background_color;
    }

    let (_cols, rows) = terminal::size()?; 
    let target_row = rows.saturating_sub(lines_from_bottom);
    let bg_color = Color::Rgb { r: color[0], g: color[1], b: color[2] };

    execute!(
        stdout,
        cursor::MoveTo(0, target_row), 
        SetBackgroundColor(bg_color),

    )?;

    println!("{}", message);

    Ok(())
}


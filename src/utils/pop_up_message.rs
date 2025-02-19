use std::io::{stdout, Result, Stdout};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor, PrintStyledContent, Stylize},
    terminal, cursor,
};

// to clear a pop up message
pub fn clear_message(stdout: &mut Stdout, lines_from_bottom: u16) -> Result<()> {
    let (_cols, rows) = terminal::size()?; 
    let target_row = rows.saturating_sub(lines_from_bottom);
    let bg_color = Color::Rgb { r: 40, g: 40, b: 40 };

    execute!(
        stdout,
        cursor::MoveTo(0, target_row), 
        SetBackgroundColor(bg_color),
        terminal::Clear(terminal::ClearType::CurrentLine), 
    )?;

    Ok(())
}


use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{self, Event};
use color_eyre::Result;

pub fn run(mut terminal: DefaultTerminal, message: &str) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, message))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, message: &str) {
    let block = ratatui::widgets::Paragraph::new(message);
    frame.render_widget(block, frame.area());
}

use crate::App;
use crate::app::AppView;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
};

use crossterm::event::{self, KeyEvent, KeyCode};  

impl App {
    pub fn auth(&mut self) -> io::Result<String> {
        let stdout = io::stdout();
        let stdout = stdout.lock();

        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        let mut textarea1 = TextArea::default();
        textarea1.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Server address")
            .border_style(Style::default().fg(Color::LightBlue)),
        );

        let mut textarea2 = TextArea::default();
        textarea2.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Username")
            .border_style(Style::default().fg(Color::LightBlue)),
        );

        let mut textarea3 = TextArea::default();
        textarea3.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Password")
            .border_style(Style::default().fg(Color::LightBlue)),
        );
        textarea3.set_mask_char('\u{2022}');

        // display 
        let size = term.size()?;
        let search_area = Rect {
            x: (size.width - size.width / 2) / 2,
            y: (size.height - 3) / 2,
            width: size.width / 2,
            height: 3,
        };

        let mut textareas = vec![textarea1, textarea2, textarea3];
        let mut current_index = 0;

        loop {
            term.draw(|f| {
                f.render_widget(&textareas[current_index], search_area);
            })?;

            match crossterm::event::read()? {
                event::Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                    if current_index < textareas.len() - 1 {
                        current_index += 1;
                    } else {
                        break; 
                    }
                }
                
                event::Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                    break; 
                }
                
                event::Event::Key(input) => {
                    if let Some(active_textarea) = textareas.get_mut(current_index) {
                        active_textarea.input(input); 
                    }
                }
                _ => {}
            }
        }

        
        if let Some(active_textarea) = textareas.get(current_index) {
            Ok(active_textarea.lines().join("\n")) 
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid textarea"))
        }
    }
}


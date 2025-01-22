use crate::api::auth::login;
use crate::api::libraries::get_continue_listening;
use crate::config::load_config;
use color_eyre::Result;
use ratatui::init;
use color_eyre::eyre::Report;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};
use ratatui::widgets::Wrap;


pub struct App {
   pub should_exit: bool,
   pub token: Option<String>,
   pub titles: Vec<String>,
   pub list_state: ListState,
}

/// Init, handlling events and navigation
 impl App {
   pub  async fn new() -> Result<Self> {
        let config = load_config()?;
        let token =
            login(&config.credentials.id.to_string(), &config.credentials.password.to_string())
                .await?;
        let titles = get_continue_listening(&token).await?;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            should_exit: false,
            token: Some(token),
            titles,
            list_state,
        })
    }

    // handle events
   pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

   /// handle key
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {}
        }
    }

    /// selection
   pub fn select_next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.titles.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

   pub fn select_previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => if i == 0 { self.titles.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }

   pub fn select_first(&mut self) {
        self.list_state.select(Some(0));
    }

  pub fn select_last(&mut self) { 
        self.list_state.select(Some(self.titles.len() - 1));
    }
}


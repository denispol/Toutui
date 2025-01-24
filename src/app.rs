use crate::api::get_test::get_test;
use crate::api::utils::collect::*;
use crate::api::auth::login;
use crate::api::libraries::get_continue_listening;
use crate::api::library_item::play;
use crate::config::load_config;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::{
         ListState    },
    DefaultTerminal,
};
use tokio::task;


pub struct App {
   pub should_exit: bool,
   pub token: Option<String>,
   pub list_state: ListState,
   pub titles: Vec<String>,
   pub authors_names: Vec<String>,
   pub ids_library_items: Vec<String>,
}

/// Init, handlling events and navigation
 impl App {
     pub  async fn new() -> Result<Self> {
         let config = load_config()?;
         let token =
             login(&config.credentials.id.to_string(), &config.credentials.password.to_string())
             .await?;

         let continue_listening = get_continue_listening(&token).await?;
         let titles = collect_titles(&continue_listening).await;
         let authors_names = collect_author_name(&continue_listening).await;
         let ids_library_items = collect_ids_library_items(&continue_listening).await;

        // test
        if let Err(e) = get_test().await {
            eprintln!("Failed to get sessions: {}", e);
        }


        let mut list_state = ListState::default(); // init the ListState ratatui's widget
        list_state.select(Some(0)); // select the first item of the list when app is launch

        Ok(Self {
            should_exit: false,
            token: Some(token),
            list_state,
            titles,
            authors_names,
            ids_library_items,
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
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                tokio::spawn(async {
                    if let Err(e) = play().await {
                        eprintln!("Error during playback: {:?}", e);
                    }
                });}

                _ => {}
            }
        }

    /// selection
    // all select fun are from ListState widget
   pub fn select_next(&mut self) {
       self.list_state.select_next();
    }

   pub fn select_previous(&mut self) {
       self.list_state.select_previous();
    }

   pub fn select_first(&mut self) {
       self.list_state.select_first();
    }

  pub fn select_last(&mut self) { 
       self.list_state.select_last();
    }
}


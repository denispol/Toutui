use crate::api::get_test::get_test;
use crate::api::libraries::PersonalizedView;
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
   pub titles: Vec<String>,
   pub authors_names: Vec<String>,
   pub list_state: ListState,
}

/// Init, handlling events and navigation
 impl App {
   pub  async fn new() -> Result<Self> {
        let config = load_config()?;
        let token =
            login(&config.credentials.id.to_string(), &config.credentials.password.to_string())
                .await?;

       let continue_listening = get_continue_listening(&token).await?;

       pub async fn collect_titles(continue_listening: &[PersonalizedView]) -> Vec<String> {
           let mut titles = Vec::new();  // Vecteur pour stocker les titres

           for library in continue_listening {
               if let Some(entities) = &library.entities {
                   for entity in entities {
                       // Déstructuration correcte de l'Option<Media>
                       if let Some(media) = &entity.media {  // Si 'media' est Some
                           if let Some(metadata) = &media.metadata { // Vérification que metadata existe
                               if let Some(title) = &metadata.title { // Vérification que title existe
                                   titles.push(title.clone()); // Ajout du titre à la liste
                               }
                           }
                       }
                   }
               }
           }

           titles  // Retourner le vecteur de titres
       }

let titles = collect_titles(&continue_listening).await;

       pub async fn collect_author_name(continue_listening: &[PersonalizedView]) -> Vec<String> {
           let mut authors_names = Vec::new();  // Vecteur pour stocker les titres

           for library in continue_listening {
               if let Some(entities) = &library.entities {
                   for entity in entities {
                       // Déstructuration correcte de l'Option<Media>
                       if let Some(media) = &entity.media {  // Si 'media' est Some
                           if let Some(metadata) = &media.metadata { // Vérification que metadata existe
                               if let Some(author_name) = &metadata.author_name { // Vérification que title existe
                                   authors_names.push(author_name.clone()); // Ajout du titre à la liste
                               }
                           }
                       }
                   }
               }
           }

           authors_names  // Retourner le vecteur de titres
       }

let authors_names = collect_author_name(&continue_listening).await;

        // test
        if let Err(e) = get_test().await {
            eprintln!("Failed to get sessions: {}", e);
        }


        let mut list_state = ListState::default(); // init the ListState ratatui's widget
        list_state.select(Some(0)); // select the first item of the list when app is launch

        Ok(Self {
            should_exit: false,
            token: Some(token),
            titles,
            authors_names,
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


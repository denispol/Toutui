//use crate::api::get_test::get_test;
use crate::api::utils::collect_personalized_view::*;
use crate::api::utils::collect_get_all_books::*;
use crate::api::utils::collect_get_pod_ep::*;
use crate::api::libraries::get_library_perso_view::*;
use crate::api::libraries::get_all_books::*;
use crate::api::library_items::get_pod_ep::*;
use crate::api::server::auth::*;
use crate::logic::handle_input::handle_l::*;
use crate::config::load_config;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::{ListState},
    DefaultTerminal,
};


pub enum AppView {
    Home,
    Library,
    SearchBook,
    PodcastEpisode,
}

pub struct App {
   pub view_state: AppView,
   pub should_exit: bool,
   pub token: Option<String>,
   pub list_state_cnt_list: ListState,
   pub list_state_library: ListState,
   pub list_state_search_results: ListState,
   pub list_state_pod_ep: ListState,
   pub titles_cnt_list: Vec<String>,
   pub auth_names_cnt_list: Vec<String>,
   pub ids_cnt_list: Vec<String>,
   pub titles_library: Vec<String>,
   pub ids_library: Vec<String>,
   pub auth_names_library: Vec<String>,
   pub ids_search_book: Vec<String>,
   pub search_query: String,
   pub search_mode: bool,
   pub is_podcast: bool,
   pub all_titles_pod_ep: Vec<Vec<String>>,
   pub all_ids_pod_ep: Vec<Vec<String>>,
   pub titles_pod_ep: Vec<String>,
   pub ids_pod_ep: Vec<String>,
}

/// Init app
 impl App {
     pub async fn new() -> Result<Self> {
         let config = load_config()?;
         let token =
             login(&config.credentials.id.to_string(), &config.credentials.password.to_string())
             .await?;

         // init for  `Home` (continue listening)
         let continue_listening = get_continue_listening(&token).await?;
         let titles_cnt_list = collect_titles_cnt_list(&continue_listening).await;
         let auth_names_cnt_list = collect_auth_names_cnt_list(&continue_listening).await;
         let ids_cnt_list = collect_ids_cnt_list(&continue_listening).await;

         //init for `Library ` (all books of a shelf)
         let all_books = get_all_books(&token).await?;
         let titles_library = collect_titles_library(&all_books).await;
         let ids_library = collect_ids_library(&all_books).await;
         let auth_names_library = collect_auth_names_library(&all_books).await;

         // init for `SearchBook`
         let ids_search_book: Vec<String> = Vec::new();
         let search_mode = false;
         let search_query = "  ".to_string();

         //init for `PodcastEpisode`
         let mut all_titles_pod_ep: Vec<Vec<String>> = Vec::new();
         let mut all_ids_pod_ep: Vec<Vec<String>> = Vec::new();
         let titles_pod_ep: Vec<String> = Vec::new();
         let ids_pod_ep: Vec<String> = Vec::new();
 


         for i in 0..ids_library.len() 
         {let podcast_episode = get_pod_ep(&token, ids_library[i].as_str()).await?;
         let title = collect_titles_pod_ep(&podcast_episode).await;
         all_titles_pod_ep.push(title);
         let id = collect_ids_pod_ep(&podcast_episode).await;
         all_ids_pod_ep.push(id);
         }
         // init for `Shelf`
         let is_podcast = true;

         // Default view_state
         let view_state = AppView::Home; // By default, Home will be the first AppView launched when the app start

         // Init ListeState for `Home` list (continue listening)
         let mut list_state_cnt_list = ListState::default(); // init the ListState ratatui's widget
         list_state_cnt_list.select(Some(0)); // select the first item of the list when app is launch

         // Init ListeState for `Library` list
         let mut list_state_library = ListState::default(); 
         list_state_library.select(Some(0)); 
                                             
         // Init ListeState for `SearchBook` list
         let mut list_state_search_results = ListState::default(); 
         list_state_search_results.select(Some(0)); 

         // Init ListState for `PodacastEpisode` list
         let mut list_state_pod_ep = ListState::default();
         list_state_pod_ep.select(Some(0));


        Ok(Self {
            should_exit: false,
            token: Some(token),
            list_state_cnt_list,
            list_state_library,
            list_state_search_results,
            list_state_pod_ep,
            titles_cnt_list,
            auth_names_cnt_list,
            ids_cnt_list,
            view_state,
            titles_library,
            ids_library,
            auth_names_library,
            ids_search_book,
            search_mode,
            search_query,
            is_podcast,
            all_titles_pod_ep,
            all_ids_pod_ep,
            titles_pod_ep,
            ids_pod_ep,
        })
    }

   /// handle events
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
        KeyCode::Char('s') => {
            let _ = self.search_active();
        }
        KeyCode::Tab => self.toggle_view(),
        KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
        KeyCode::Char('j') | KeyCode::Down => self.select_next(),
        KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => self.select_first(),
        KeyCode::Char('G') | KeyCode::End => self.select_last(),
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            // Clone needed because variables will be used in a spawn
            let token = self.token.clone();
            let port = "1234".to_string();

            // Init for `Continue Listening`
            let ids_cnt_list = self.ids_cnt_list.clone();
            let selected_cnt_list = self.list_state_cnt_list.selected();

            // Init for `Library`
            let ids_library = self.ids_library.clone();
            let selected_library = self.list_state_library.selected();

            // Init for `Search Book`
            let ids_search_book = self.ids_search_book.clone();
            let selected_search_book = self.list_state_search_results.selected();

            // Init for `PodcastEpisode`
            let ids_pod_ep = self.ids_pod_ep.clone();
            let selected_pod_ep = self.list_state_pod_ep.selected();

            let id_test =  "cac";

            // Now, spawn the async task based on the current view state
            match self.view_state {
                AppView::Home => {
                    tokio::spawn(async move {
                        handle_l(token.as_ref(), &ids_cnt_list, selected_cnt_list, port, &id_test).await;
                    });
                }
                AppView::Library => {
                    if self.is_podcast {
                    if let Some(index) = selected_library {
                        self.titles_pod_ep = self.all_titles_pod_ep[index].clone();
                        self.list_state_pod_ep.select(Some(0));
                        self.view_state = AppView::PodcastEpisode;
                    }} else {
                        tokio::spawn(async move {
                            handle_l(token.as_ref(), &ids_library, selected_library, port, &id_test).await;
                        });
                    }
                }
                AppView::SearchBook => {
                    tokio::spawn(async move {
                        handle_l(token.as_ref(), &ids_search_book, selected_search_book, port, &id_test).await;
                    });
                }
                AppView::PodcastEpisode => {
                    if let Some(index) = selected_library {
                        if let Some(id_pod) = ids_library.get(index) {
                            let all_ids_pod_ep_clone = self.all_ids_pod_ep.clone();
                            let id_pod_clone = id_pod.clone();
                            tokio::spawn(async move {
                                handle_l(token.as_ref(), &all_ids_pod_ep_clone[index], selected_pod_ep, port, id_pod_clone.as_str()).await;
                            });
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

    /// Toggle between Home and Library views
    fn toggle_view(&mut self) {
        self.view_state = match self.view_state {
            AppView::Home => AppView::Library,
            AppView::Library => AppView::Home,
            AppView::SearchBook => AppView::Home,
            AppView::PodcastEpisode => AppView::Home,

        };
    }

    /// Select functions that apply to both views
    /// all select functions are from ListState widget
    pub fn select_next(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_next(),
            AppView::Library => self.list_state_library.select_next(),
            AppView::SearchBook => self.list_state_search_results.select_next(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_next(),
        }
    }

    pub fn select_previous(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_previous(),
            AppView::Library => self.list_state_library.select_previous(),
            AppView::SearchBook => self.list_state_search_results.select_previous(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_previous(),
        }
    }

    pub fn select_first(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_first(),
            AppView::Library => self.list_state_library.select_first(),
            AppView::SearchBook => self.list_state_search_results.select_first(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_first(),
        }
    }

    pub fn select_last(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_last(),
            AppView::Library => self.list_state_library.select_last(),
            AppView::SearchBook => self.list_state_search_results.select_last(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_last(),
        }
    }

 }

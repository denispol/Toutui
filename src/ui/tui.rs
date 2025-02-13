use crate::App;
use crate::app::AppView;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem , ListState,  Paragraph, StatefulWidget,
        Widget, Wrap
    },
};

// Auth
use tui_textarea::{Input, Key, TextArea};
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;





// const for color theme
const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

// const version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// init widget for selected AppView 
impl Widget for &mut App {
  fn render(self, area: Rect, buf: &mut Buffer) {
        match self.view_state {
            AppView::Home => self.render_home(area, buf),
            AppView::Library => self.render_library(area, buf),
            AppView::SearchBook => self.render_search_book(area, buf),
            AppView::PodcastEpisode => self.render_pod_ep(area, buf),
            AppView::Settings => self.render_settings(area, buf),
            AppView::SettingsAccount => self.render_settings_account(area, buf),
            AppView::SettingsLibrary => self.render_settings_library(area, buf),
        }
    }
}


/// Rendering logic

impl App {
    /// AppView::Home rendering
    fn render_home(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, refresh_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area1, item_area2] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Continue Listening";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.titles_cnt_list.clone(), &mut self.list_state_cnt_list.clone());
        self.render_info_home(item_area1, buf, &mut self.list_state_cnt_list.clone());
        self.render_desc_home(item_area2, buf, &mut self.list_state_cnt_list.clone());
    }

    /// AppView::Library rendering
    fn render_library(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, refresh_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area1, item_area2] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Library";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.titles_library.clone(), &mut self.list_state_library.clone());
        self.render_info_library(item_area1, buf, &mut self.list_state_library.clone());
        self.render_desc_library(item_area2, buf, &mut self.list_state_library.clone());
    }

    /// AppView::Settings rendering
    fn render_settings(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1),]).areas(main_area);

        let render_list_title = "Settings";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.settings.clone(), &mut self.list_state_settings.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

    /// AppView::SettingsAccount rendering
    fn render_settings_account(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1),]).areas(main_area);

        let render_list_title = "Settings account";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.all_usernames.clone(), &mut &mut self.list_state_settings_account.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

    /// AppView::SettingsLibrary rendering
    fn render_settings_library(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1),]).areas(main_area);

        let render_list_title = "Settings library";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, &self.libraries_names.clone(), &mut &mut self.list_state_settings_library.clone());
        //self.render_selected_item(item_area, buf, &self.titles_library.clone(), self.auth_names_library.clone());
    }

    /// AppView::SearchBook rendering
    fn render_search_book(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, refresh_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area1, item_area2] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Search result";
        let text_render_footer = "Use Tab to back home, ↓↑ to move, → to play, s to search, q to quit.";

        if self.search_mode {
            if let Ok(query) = self.search_active() {
                self.search_query = query.to_string();
                self.search_mode = false; 
            }
        }

        // init variables for search result (search by a book by title)
        let idx_and_titles: Vec<(usize, String)> = self.titles_library
            .iter()
            .enumerate() 
            .filter(|(_, x)| x.to_lowercase().contains(&self.search_query.to_lowercase())) 
            .map(|(index, title)| (index, title.clone())) 
            .collect();

        let mut titles_search_book_or_pod: Vec<String> = Vec::new();
        let mut index_to_keep: Vec<usize> = Vec::new();
        for (index, title) in idx_and_titles {
            titles_search_book_or_pod.push(title.to_string());
            index_to_keep.push(index)
        }

        let titles_search_book_or_pod: &[String] = &titles_search_book_or_pod;

        // for book
        self.ids_search_book = self.ids_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.auth_names_pod_search_book = self.auth_names_library_pod
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.auth_names_search_book = self.auth_names_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.published_year_library_search_book = self.published_year_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.desc_library_search_book = self.desc_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.duration_library_search_book = self.duration_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();

        // for podacst
        self.all_titles_pod_ep_search = self.all_titles_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_ids_pod_ep_search = self.all_ids_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_subtitles_pod_ep_search = self.all_subtitles_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_seasons_pod_ep_search = self.all_seasons_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_episodes_pod_ep_search = self.all_episodes_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_authors_pod_ep_search = self.all_authors_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_descs_pod_ep_search = self.all_descs_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_titles_pod_search = self.all_titles_pod
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.all_durations_pod_ep_search = self.all_durations_pod_ep
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();
        self.ids_library_pod_search = self.ids_library
            .iter()
            .enumerate()
            .filter(|(index, _)| index_to_keep.contains(&index))
            .map(|(_, value)| value.clone())
            .collect();

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        self.render_list(list_area, buf, render_list_title, titles_search_book_or_pod, &mut self.list_state_search_results.clone());
        self.render_info_search_book(item_area1, buf, &mut &self.list_state_search_results.clone());
        self.render_desc_search_book(item_area2, buf, &mut &self.list_state_search_results.clone());

    }

    /// AppView::PodcastEpisode
    fn render_pod_ep(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, refresh_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        let [list_area, item_area1, item_area2] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)]).areas(main_area);

        let render_list_title = "Episodes";
        let text_render_footer = "Use ↓↑ to move, → to play, s to search, q to quit.";

        App::render_header(header_area, buf, self.lib_name_type.clone(), &self.username, &self.server_address, VERSION);
        App::render_footer(footer_area, buf, text_render_footer);
        if self.is_from_search_pod {
        self.render_list(list_area, buf, render_list_title, &self.titles_pod_ep_search.clone(), &mut self.list_state_pod_ep.clone());
        self.render_info_pod_ep_search(item_area1, buf, &mut &self.list_state_pod_ep.clone());
        self.render_desc_pod_ep_search(item_area2, buf, &mut &self.list_state_pod_ep.clone());
        } else {
        self.render_list(list_area, buf, render_list_title, &self.titles_pod_ep.clone(), &mut self.list_state_pod_ep.clone());
        self.render_info_pod_ep(item_area1, buf, &mut &self.list_state_pod_ep.clone());
        self.render_desc_pod_ep(item_area2, buf, &mut &self.list_state_pod_ep.clone());
        }
    }

    /// General functions for rendering 

    fn render_header(area: Rect, buf: &mut Buffer, library_name: String, username: &str, server_address: &str, version: &str) {
        Paragraph::new(library_name)
            .bold()
            .centered()
            .render(area, buf);
        Paragraph::new(format!("Connected as {}\n{}", &username, &server_address))
            .not_bold()
            .left_aligned()
            .render(area, buf);
        Paragraph::new(format!("Toutui v{}", version))
            .right_aligned()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer, text_render_footer: &str) {
        Paragraph::new(text_render_footer)
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer, render_list_title: &str, render_list_items: &[String], list_state: &mut ListState) {
        let block = Block::new()
            .title(Line::raw(format!("{}", render_list_title)).centered())
            .borders(Borders::TOP)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = render_list_items
            .iter()
            .enumerate()
            .map(|(i, title)| {
                let color = Self::alternate_colors(i);
                ListItem::new(title.clone()).bg(color)
            })
        .collect();


        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, list_state);
    }


    // info about the book or podacst for `Home`
    fn render_info_home(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {
            if self.is_podcast {
            Paragraph::new(format!("[{}] - Author: {} - Episode: {} - Duration: {}", 
                    self.titles_pod_cnt_list[selected], 
                    self.authors_pod_cnt_list[selected], 
                    self.nums_ep_pod_cnt_list[selected],
                    self.durations_pod_cnt_list[selected],
                    ))
                .left_aligned()
                .render(area, buf);
            } else {
            Paragraph::new(format!("Author: {} - Year: {} - Duration: {}\nProgress: {}%, {}, {}", 
                    self.auth_names_cnt_list[selected], 
                    self.pub_year_cnt_list[selected], 
                    self.duration_cnt_list[selected],
                    self.book_progress[selected][0], // precentage progression
                    self.book_progress[selected][2], // time left
                    self.book_progress[selected][1], // is finished
                    ))
                .left_aligned()
                .render(area, buf);
            }
        }
    }

    // description of the book or podcast `Home`
    fn render_desc_home(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {
            let mut content: String = String::new();
            if self.is_podcast {
            content = self.subtitles_pod_cnt_list[selected].clone();
            } else {
            content = self.desc_cnt_list[selected].clone();
            }

            Paragraph::new(content.clone())
                .scroll((self.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }

    // info about the book or podacst for `Library`
    fn render_info_library(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {
            if self.is_podcast {
            Paragraph::new(format!("Author: {}", 
                    self.auth_names_library_pod[selected], 
                    ))
                .left_aligned()
                .render(area, buf);
            } 
            else {
            Paragraph::new(format!("Author: {} - Year: {} - Duration: {}", 
                    self.auth_names_library[selected], 
                    self.published_year_library[selected], 
                    self.duration_library[selected]))
                .left_aligned()
                .render(area, buf);
            }
        }
    }

    // description of the book or podcast `Library`
    fn render_desc_library(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {

            Paragraph::new(self.desc_library[selected].clone())
                .scroll((self.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }

    // info about the podcast for `PodcastEpisode`
    fn render_info_pod_ep(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        let n = self.durations_pod_ep.len();
        let duplicated_titles = vec![self.titles_pod[0].clone(); n];
        let duplicated_authors = vec![self.authors_pod_ep[0].clone(); n];
        if let Some(selected) = list_state.selected() {
            
            Paragraph::new(format!("[{}] - Author: {} - Episode: {} - Duration: {} ", 
                    duplicated_titles[selected].trim(), 
                    duplicated_authors[selected].trim(), 
                    self.episodes_pod_ep[selected].trim(),
                    self.durations_pod_ep[selected].trim(),
                    ))
                .left_aligned()
                .render(area, buf);
        }
    }
    // info about the podcast for `PodcastEpisode` (from search)
    fn render_info_pod_ep_search(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        let n = self.durations_pod_ep_search.len();
        let duplicated_titles_search = vec![self.titles_pod_search[0].clone(); n];
        let duplicated_authors_search = vec![self.authors_pod_ep_search[0].clone(); n];
        if let Some(selected) = list_state.selected() {
            
            Paragraph::new(format!("[{}] - Author: {} - Episode: {} - Duration: {} ", 
                    duplicated_titles_search[selected].trim(), 
                    duplicated_authors_search[selected].trim(), 
                    self.episodes_pod_ep_search[selected].trim(),
                    self.durations_pod_ep_search[selected].trim(),
                    ))
                .left_aligned()
                .render(area, buf);
        }
    }

    // desc of the podcast for `PodcastEpisode`
    fn render_desc_pod_ep(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {

            Paragraph::new(self.subtitles_pod_ep[selected].clone())
                .scroll((self.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
    // desc of the podcast for `PodcastEpisode` (from search)
    fn render_desc_pod_ep_search(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {

            Paragraph::new(self.subtitles_pod_ep_search[selected].clone())
                .scroll((self.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }

    // info about the book or podacst for `SearchBook`
    fn render_info_search_book(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {
            if self.is_podcast {
            Paragraph::new(format!("Author: {}", 
                    self.auth_names_pod_search_book[selected], 
                    ))
                .left_aligned()
                .render(area, buf);
            } 
            else {
            Paragraph::new(format!("Author: {} - Year: {} - Duration: {}", 
                    self.auth_names_search_book[selected], 
                    self.published_year_library_search_book[selected], 
                    self.duration_library_search_book[selected]))
                .left_aligned()
                .render(area, buf);
            }
        }
    }

    // description of the book or podcast `SearchBook`
    fn render_desc_search_book(&self, area: Rect, buf: &mut Buffer, list_state: &ListState) {

        if let Some(selected) = list_state.selected() {

            Paragraph::new(self.desc_library_search_book[selected].clone())
                .scroll((self.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
    const fn alternate_colors(i: usize) -> Color {
        if i % 2 == 0 {
            NORMAL_ROW_BG
        } else {
            ALT_ROW_BG_COLOR
        }
    }
}

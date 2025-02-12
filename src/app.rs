use crate::api::utils::collect_personalized_view::*;
use crate::api::utils::collect_personalized_view_pod::*;
use crate::api::utils::collect_get_all_books::*;
use crate::api::utils::collect_get_pod_ep::*;
use crate::api::utils::collect_get_all_libraries::*;
use crate::api::libraries::get_library_perso_view::*;
use crate::api::libraries::get_library_perso_view_pod::*;
use crate::api::libraries::get_all_books::*;
use crate::api::libraries::get_all_libraries::*;
use crate::api::library_items::get_pod_ep::*;
use crate::logic::handle_input::handle_l_book::*;
use crate::logic::handle_input::handle_l_pod::*;
use crate::logic::handle_input::handle_l_pod_home::*;
use crate::main;
use crate::config::load_config;
use crate::db::crud::*;
use crate::db::database_struct::Database;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::{ListState},
    DefaultTerminal,
};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use std::thread;
use std::time::Duration;
use std::process;
use std::io::{stdout, Write};
use crossterm::{cursor, execute, terminal};

pub enum AppView {
    Home,
    Library,
    SearchBook,
    PodcastEpisode,
    Settings,
    SettingsAccount,
    SettingsLibrary,
}

pub struct App {
   pub view_state: AppView,
   pub database: Database,
   pub id_selected_lib: String,
   pub token: Option<String>,
   pub should_exit: bool,
   pub list_state_cnt_list: ListState,
   pub list_state_library: ListState,
   pub list_state_search_results: ListState,
   pub list_state_pod_ep: ListState,
   pub list_state_settings: ListState,
   pub list_state_settings_account: ListState,
   pub list_state_settings_library: ListState,
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
   pub ids_ep_cnt_list: Vec<String>,
   pub all_titles_pod_ep_search: Vec<Vec<String>>,
   pub titles_pod_ep_search: Vec<String>,
   pub is_from_search_pod: bool,
   pub ids_library_pod_search: Vec<String>,
   pub all_ids_pod_ep_search: Vec<Vec<String>>,
   pub libraries_names: Vec<String>,
   pub media_types: Vec<String>,
   pub libraries_ids: Vec<String>,
   pub library_name: String,
   pub media_type: String,
   pub lib_name_type: String,
   pub settings: Vec<String>,
   pub all_usernames: Vec<String>,
   pub all_server_addresses: Vec<String>,
   pub username: String,
   pub server_address: String,
   pub scroll_offset: u16,
   pub max_scroll: usize,
   pub lorme: String,
}

/// Init app
impl App {
    pub async fn new() -> Result<Self> {
            let lorme = r#"Quod opera consulta cogitabatur astute, ut hoc insidiarum genere Galli periret avunculus, ne eum ut praepotens acueret in fiduciam exitiosa coeptantem. verum navata est opera diligens hocque dilato Eusebius praepositus cubiculi missus est Cabillona aurum secum perferens, quo per turbulentos seditionum concitores occultius distributo et tumor consenuit militum et salus est in tuto locata praefecti. deinde cibo abunde perlato castra die praedicto sunt mota.
Quaestione igitur per multiplices dilatata fortunas cum ambigerentur quaedam, non nulla levius actitata constaret, post multorum clades Apollinares ambo pater et filius in exilium acti cum ad locum Crateras nomine pervenissent, villam scilicet suam quae ab Antiochia vicensimo et quarto disiungitur lapide, ut mandatum est, fractis cruribus occiduntur.
Et eodem impetu Domitianum praecipitem per scalas itidem funibus constrinxerunt, eosque coniunctos per ampla spatia civitatis acri raptavere discursu. iamque artuum et membrorum divulsa conpage superscandentes corpora mortuorum ad ultimam truncata deformitatem velut exsaturati mox abiecerunt in flumen.
Incenderat autem audaces usque ad insaniam homines ad haec, quae nefariis egere conatibus, Luscus quidam curator urbis subito visus: eosque ut heiulans baiolorum praecentor ad expediendum quod orsi sunt incitans vocibus crebris. qui haut longe postea ideo vivus exustus est.
Eodem tempore Serenianus ex duce, cuius ignavia populatam in Phoenice Celsen ante rettulimus, pulsatae maiestatis imperii reus iure postulatus ac lege, incertum qua potuit suffragatione absolvi, aperte convictus familiarem suum cum pileo, quo caput operiebat, incantato vetitis artibus ad templum misisse fatidicum, quaeritatum expresse an ei firmum portenderetur imperium, ut cupiebat, et cunctum.
Novo denique perniciosoque exemplo idem Gallus ausus est inire flagitium grave, quod Romae cum ultimo dedecore temptasse aliquando dicitur Gallienus, et adhibitis paucis clam ferro succinctis vesperi per tabernas palabatur et conpita quaeritando Graeco sermone, cuius erat inpendio gnarus, quid de Caesare quisque sentiret. et haec confidenter agebat in urbe ubi pernoctantium luminum claritudo dierum solet imitari fulgorem. postremo agnitus saepe iamque, si prodisset, conspicuum se fore contemplans, non nisi luce palam egrediens ad agenda quae putabat seria cernebatur. et haec quidem medullitus multis gementibus agebantur.
Horum adventum praedocti speculationibus fidis rectores militum tessera data sollemni armatos omnes celeri eduxere procursu et agiliter praeterito Calycadni fluminis ponte, cuius undarum magnitudo murorum adluit turres, in speciem locavere pugnandi. neque tamen exiluit quisquam nec permissus est congredi. formidabatur enim flagrans vesania manus et superior numero et ruitura sine respectu salutis in ferrum.
Montius nos tumore inusitato quodam et novo ut rebellis et maiestati recalcitrantes Augustae per haec quae strepit incusat iratus nimirum quod contumacem praefectum, quid rerum ordo postulat ignorare dissimulantem formidine tenus iusserim custodiri.
Et quoniam mirari posse quosdam peregrinos existimo haec lecturos forsitan, si contigerit, quamobrem cum oratio ad ea monstranda deflexerit quae Romae gererentur, nihil praeter seditiones narratur et tabernas et vilitates harum similis alias, summatim causas perstringam nusquam a veritate sponte propria digressurus.
Dumque ibi diu moratur commeatus opperiens, quorum translationem ex Aquitania verni imbres solito crebriores prohibebant auctique torrentes, Herculanus advenit protector domesticus, Hermogenis ex magistro equitum filius, apud Constantinopolim, ut supra rettulimus, populari quondam turbela discerpti. quo verissime referente quae Gallus egerat, damnis super praeteritis maerens et futurorum timore suspensus angorem animi quam diu potuit emendabat.
Ut enim quisque sibi plurimum confidit et ut quisque maxime virtute et sapientia sic munitus est, ut nullo egeat suaque omnia in se ipso posita iudicet, ita in amicitiis expetendis colendisque maxime excellit. Quid enim? Africanus indigens mei? Minime hercule! ac ne ego quidem illius; sed ego admiratione quadam virtutis eius, ille vicissim opinione fortasse non nulla, quam de meis moribus habebat, me dilexit; auxit benevolentiam consuetudo. Sed quamquam utilitates multae et magnae consecutae sunt, non sunt tamen ab earum spe causae diligendi profectae.
Latius iam disseminata licentia onerosus bonis omnibus Caesar nullum post haec adhibens modum orientis latera cuncta vexabat nec honoratis parcens nec urbium primatibus nec plebeiis.
Haec et huius modi quaedam innumerabilia ultrix facinorum impiorum bonorumque praemiatrix aliquotiens operatur Adrastia atque utinam semper quam vocabulo duplici etiam Nemesim appellamus: ius quoddam sublime numinis efficacis, humanarum mentium opinione lunari circulo superpositum, vel ut definiunt alii, substantialis tutela generali potentia partilibus praesidens fatis, quam theologi veteres fingentes Iustitiae filiam ex abdita quadam aeternitate tradunt omnia despectare terrena.
Quam ob rem circumspecta cautela observatum est deinceps et cum edita montium petere coeperint grassatores, loci iniquitati milites cedunt. ubi autem in planitie potuerint reperiri, quod contingit adsidue, nec exsertare lacertos nec crispare permissi tela, quae vehunt bina vel terna, pecudum ritu inertium trucidantur.
Etenim si attendere diligenter, existimare vere de omni hac causa volueritis, sic constituetis, iudices, nec descensurum quemquam ad hanc accusationem fuisse, cui, utrum vellet, liceret, nec, cum descendisset, quicquam habiturum spei fuisse, nisi alicuius intolerabili libidine et nimis acerbo odio niteretur. Sed ego Atratino, humanissimo atque optimo adulescenti meo necessario, ignosco, qui habet excusationem vel pietatis vel necessitatis vel aetatis. Si voluit accusare, pietati tribuo, si iussus est, necessitati, si speravit aliquid, pueritiae. Ceteris non modo nihil ignoscendum, sed etiam acriter est resistendum.
Montius nos tumore inusitato quodam et novo ut rebellis et maiestati recalcitrantes Augustae per haec quae strepit incusat iratus nimirum quod contumacem praefectum, quid rerum ordo postulat ignorare dissimulantem formidine tenus iusserim custodiri.
Sed ut tum ad senem senex de senectute, sic hoc libro ad amicum amicissimus scripsi de amicitia. Tum est Cato locutus, quo erat nemo fere senior temporibus illis, nemo prudentior; nunc Laelius et sapiens (sic enim est habitus) et amicitiae gloria excellens de amicitia loquetur. Tu velim a me animum parumper avertas, Laelium loqui ipsum putes. C. Fannius et Q. Mucius ad socerum veniunt post mortem Africani; ab his sermo oritur, respondet Laelius, cuius tota disputatio est de amicitia, quam legens te ipse cognosces.
Procedente igitur mox tempore cum adventicium nihil inveniretur, relicta ora maritima in Lycaoniam adnexam Isauriae se contulerunt ibique densis intersaepientes itinera praetenturis provincialium et viatorum opibus pascebantur.
Quam ob rem ut ii qui superiores sunt submittere se debent in amicitia, sic quodam modo inferiores extollere. Sunt enim quidam qui molestas amicitias faciunt, cum ipsi se contemni putant; quod non fere contingit nisi iis qui etiam contemnendos se arbitrantur; qui hac opinione non modo verbis sed etiam opere levandi sunt.
Quare talis improborum consensio non modo excusatione amicitiae tegenda non est sed potius supplicio omni vindicanda est, ut ne quis concessum putet amicum vel bellum patriae inferentem sequi; quod quidem, ut res ire coepit, haud scio an aliquando futurum sit. Mihi autem non minori curae est, qualis res publica post mortem meam futura, quam qualis hodie sit.
"#.to_string();


        // init config
        let config = load_config()?;

        let mut view_state = AppView::Home; // By default, Home will be the first AppView launched when the app start
        // Default view_state
        // init database from Database struct
        let mut database = Database::new().await?;
                                        
        // init token 
        let mut token: String = String::new();
        if let Some(var_token) = database.default_usr.get(2) {
            token = var_token.clone();

        }

        // init id_selected_lib
        let mut id_selected_lib: String = String::new();
        if let Some(var_id_selected_lib) = database.default_usr.get(5) {
            id_selected_lib = var_id_selected_lib.clone();

        }

        // init current username
        let mut username: String = String::new();
        if let Some(var_username) = database.default_usr.get(0) {
            username = var_username.clone();

        }

        // init server address (without prefix)
        let mut server_address: String = String::new();
        if let Some(var_server_address) = database.default_usr.get(1) {
            server_address = var_server_address.clone();

            // Remove "http://" or "https://"
            if let Some(stripped) = server_address.strip_prefix("http://") {
                server_address = stripped.to_string();
            } else if let Some(stripped) = server_address.strip_prefix("https://") {
                server_address = stripped.to_string();
            }
        }

         // init for `Libraries` (get all Libraries (shelf), can be a podcast or book type)
         let all_libraries = get_all_libraries(&token).await?;
         let libraries_names = collect_library_names(&all_libraries).await; // all the libraries names of the user ex : {name1, name2}
         let media_types = collect_media_types(&all_libraries).await; // all media type of libraries ex : {book, podcast}
         let libraries_ids = collect_library_ids(&all_libraries).await; // all all libraries ids
         let mut library_name = String::new(); // library name of the selected library
         let mut media_type = String::new(); // media type of the selected library

         let target = id_selected_lib.clone();

         // retrieve name and mediatype of the current librarie
         if let Some(index) = libraries_ids.iter().position(|x| x == &target) {
             library_name = libraries_names[index].clone();
             media_type = media_types[index].clone();
         }         
         let lib_name_type = format!("{} ({})", library_name, media_type);

         // init is_podcast
         let is_podcast = if media_type == "podcast" {
             true
         } else {
             false
         };


        // init for `Home` (continue listening)
        let mut titles_cnt_list: Vec<String> = Vec::new();
        let mut auth_names_cnt_list: Vec<String> = Vec::new();
        let mut ids_cnt_list: Vec<String> = Vec::new();
        let mut ids_ep_cnt_list: Vec<String> = Vec::new();


        if is_podcast {
         // init for  `Home` (continue listening) for podcasts
         let continue_listening_pod = get_continue_listening_pod(&token).await?;
         ids_cnt_list = collect_ids_pod_cnt_list(&continue_listening_pod).await; // id of a podcast
         titles_cnt_list = collect_titles_cnt_list_pod(&continue_listening_pod).await;
         ids_ep_cnt_list = collect_ids_ep_pod_cnt_list(&continue_listening_pod).await; // id of a podcast episode
         }
         else {
         // init for  `Home` (continue listening) for books
         let continue_listening = get_continue_listening(&token).await?;
         titles_cnt_list = collect_titles_cnt_list(&continue_listening).await;
         auth_names_cnt_list = collect_auth_names_cnt_list(&continue_listening).await;
         ids_cnt_list = collect_ids_cnt_list(&continue_listening).await;
         }

         //init for `Library ` (all books  or podcasts of a Library (shelf))
         let all_books = get_all_books(&token, &id_selected_lib).await?;
         let titles_library = collect_titles_library(&all_books).await;
         let ids_library = collect_ids_library(&all_books).await;
         let auth_names_library = collect_auth_names_library(&all_books).await;


         // init for `SearchBook`
         let ids_search_book: Vec<String> = Vec::new();
         let search_mode = false;
         let search_query = "  ".to_string();
         let all_titles_pod_ep_search: Vec<Vec<String>> = Vec::new();
         let titles_pod_ep_search: Vec<String> = Vec::new();
         let is_from_search_pod = false;
         let ids_library_pod_search: Vec<String> = Vec::new();
         let mut all_ids_pod_ep_search: Vec<Vec<String>> = Vec::new();


         //init for `PodcastEpisode`
         let mut all_titles_pod_ep: Vec<Vec<String>> = Vec::new(); // fetch titles for all podcast episodes. Ex: {titles_pod1_ep1, title_pod1_ep2}, {titles_pod2_ep1, title_pod2_ep2} 
         let mut all_ids_pod_ep: Vec<Vec<String>> = Vec::new();
         let titles_pod_ep: Vec<String> = Vec::new(); // fetch episode titles for a podcast. {titles_pod1_ep1, title_pod1_ep2} 
         let ids_pod_ep: Vec<String> = Vec::new();

         for i in 0..ids_library.len() 
         {let podcast_episode = get_pod_ep(&token, ids_library[i].as_str()).await?;
         let title = collect_titles_pod_ep(&podcast_episode).await;
         all_titles_pod_ep.push(title);
         let id = collect_ids_pod_ep(&podcast_episode).await;
         all_ids_pod_ep.push(id);
         }

         // init for `Settings`
         let settings = vec!["Account".to_string(), "Library".to_string()];

         // init for `SettingsAccount`
         let mut all_usernames: Vec<String> = Vec::new();
         let mut all_server_addresses: Vec<String> = Vec::new();
         if let Some(var_username) = database.default_usr.get(0) {
             all_usernames.push(var_username.clone());
         }
         if let Some(var_server_address) = database.default_usr.get(1) {
             all_server_addresses.push(var_server_address.clone());
         }

         // init variables for for scrolling into description section 
         let scroll_offset = 0;
         let max_scroll = lorme.len().saturating_sub(5);



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

         // Init ListState for `Settings` list
         let mut list_state_settings = ListState::default();
         list_state_settings.select(Some(0));

         // Init ListState for `SettingsAccount` list
         let mut list_state_settings_account = ListState::default();
         list_state_settings_account.select(Some(0));

         // Init ListState for `SettingsLibrary` list
         let mut list_state_settings_library = ListState::default();
         list_state_settings_library.select(Some(0));

        Ok(Self {
            database,
            id_selected_lib,
            token: Some(token),
            should_exit: false,
            list_state_cnt_list,
            list_state_library,
            list_state_search_results,
            list_state_pod_ep,
            list_state_settings,
            list_state_settings_account,
            list_state_settings_library,
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
            ids_ep_cnt_list, 
            all_titles_pod_ep_search,
            titles_pod_ep_search,
            is_from_search_pod,
            ids_library_pod_search,
            all_ids_pod_ep_search,
            libraries_names,
            libraries_ids,
            media_types,
            library_name,
            media_type,
            lib_name_type,
            settings,
            all_usernames,
            all_server_addresses,
            username,
            server_address,
            scroll_offset,
            max_scroll,
            lorme,
        })
    }


   /// handle events
   pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
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
        KeyCode::Char('/') | KeyCode::Char(' ') => {
            let _ = self.search_active();
        }
        KeyCode::Char('S') => {
            self.view_state = AppView::Settings;
        }
        KeyCode::Tab => {
            if self.is_from_search_pod {
                self.is_from_search_pod = false;
            };
            self.toggle_view()
        }
        KeyCode::Char('Q') | KeyCode::Esc => process::exit(0), // need to exit run function once, and after
                                                       // should quit once again the run from loop main function :
                                                       // (`let result = app.run(&mut terminal);`)
        KeyCode::Char('R') => self.should_exit = true, // same as above, need to quit once before
                                                       // be able to execute `R` from main function 
        KeyCode::Char('j') | KeyCode::Down => self.select_next(),
        // scroll up into description section
        KeyCode::Char('J') =>{
            if usize::from(self.scroll_offset) < self.max_scroll {
                self.scroll_offset += 1;
            }
        }        
        // go start description section
        KeyCode::Char('H') =>{
            if usize::from(self.scroll_offset) < self.max_scroll {
                self.scroll_offset = 0;
            }
        }        
        KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
        // scroll down into description section
        KeyCode::Char('K') => {
            if usize::from(self.scroll_offset) > 0 {
                self.scroll_offset -= 1;
            }
        }
        KeyCode::Char('g') | KeyCode::Home => self.select_first(),
        KeyCode::Char('G') | KeyCode::End => self.select_last(),
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            // Clone needed because variables will be used in a spawn
            let token = self.token.clone();
            let port = "1234".to_string();

            // Init for `Continue Listening` (AppView::Home)
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
            let ids_ep_cnt_list = self.ids_ep_cnt_list.clone();

            // Init for `SettingsAccount`
            let selected_account = self.list_state_settings_account.selected();

            // Init for `SettingsLibrary`
            let selected_settings_library = self.list_state_settings_library.selected();

            // loading message 
            pub fn loading_message() {
                let mut stdout = stdout();
                if let Ok((cols, rows)) = terminal::size() {
                    execute!(stdout, cursor::MoveTo(0, rows.saturating_sub(2)));
                    println!("Loading...");
                }                         
            }


            // Now, spawn the async task based on the current view state
            match self.view_state {
                AppView::Home => {
                    if self.is_podcast {
                        loading_message();
                        tokio::spawn(async move {
                            handle_l_pod_home(token.as_ref(), &ids_cnt_list, selected_cnt_list, port, ids_ep_cnt_list).await;
                        });
                    } else {
                        loading_message();
                        tokio::spawn(async move {
                        handle_l_book(token.as_ref(), ids_cnt_list, selected_cnt_list, port).await;
                    });
                    }}
                AppView::Settings => {
                    match self.list_state_settings.selected() {
                        Some(0) => self.view_state = AppView::SettingsAccount,
                        Some(1) => self.view_state = AppView::SettingsLibrary,
                        _ => {}
                    }
                }
                AppView::SettingsAccount => {
                    if let Some(index) = selected_account {
                    let usr_to_delete = &self.all_usernames[index];
                    delete_user(usr_to_delete.as_str());
                    }
                }
                AppView::SettingsLibrary => {
                  if let Some(index) = selected_settings_library {
                    let new_selected_lib = &self.libraries_ids[index];
                    update_id_selected_lib(&new_selected_lib, &self.username);
                    }
                }
                AppView::Library => {
                    if self.is_podcast {
                    if let Some(index) = selected_library {
                        self.titles_pod_ep = self.all_titles_pod_ep[index].clone();
                        self.list_state_pod_ep.select(Some(0));
                        self.view_state = AppView::PodcastEpisode;
                    }} else {
                        loading_message();
                        tokio::spawn(async move {
                            handle_l_book(token.as_ref(), ids_library, selected_library, port).await;
                        });
                    }
                }
                AppView::SearchBook => {
                    if self.is_podcast {
                        self.is_from_search_pod = true;
                        if let Some(index) = selected_search_book {
                            self.titles_pod_ep_search = self.all_titles_pod_ep_search[index].clone();
                            self.list_state_pod_ep.select(Some(0));
                            self.view_state = AppView::PodcastEpisode;
                        }} else {   
                            loading_message();
                            tokio::spawn(async move {
                                handle_l_book(token.as_ref(), ids_search_book, selected_search_book, port).await;
                            });

                        }
                }
                AppView::PodcastEpisode => {
                    if self.is_from_search_pod {
                    // we need the index of selected_search_book to feet after with
                    // ids_library_pod_search
                    if let Some(index) = selected_search_book {
                        // ids_library_pod_search because we need the pod id and he is given by
                        // this variable
                        if let Some(id_pod) = self.ids_library_pod_search.get(index) {
                        //    println!("{:?}", id_pod);
                            let all_ids_pod_ep_search_clone = self.all_ids_pod_ep_search.clone();
                         //   println!("{:?}", all_ids_pod_ep_search_clone[index]);
                            let id_pod_clone = id_pod.clone();
                            loading_message();
                            tokio::spawn(async move {
                                handle_l_pod(token.as_ref(), &all_ids_pod_ep_search_clone[index], selected_pod_ep, port, id_pod_clone.as_str()).await;
                            });
                        }
                    }
                    } else {
                        // selected_livrary ids_library because we need the pod id and he is given by
                        // these variables
                        // we also need the index of selected library to feet after with
                        // ids_library
                    if let Some(index) = selected_library {
                        if let Some(id_pod) = ids_library.get(index) {
                            let all_ids_pod_ep_clone = self.all_ids_pod_ep.clone();
                            let id_pod_clone = id_pod.clone();
                            tokio::spawn(async move {
                                loading_message();
                                handle_l_pod(token.as_ref(), &all_ids_pod_ep_clone[index], selected_pod_ep, port, id_pod_clone.as_str()).await;
                            });
                        }
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
            AppView::Settings => AppView::Home,
            AppView::SettingsAccount => AppView::Home,
            AppView::SettingsLibrary => AppView::Home,

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
            AppView::Settings => self.list_state_settings.select_next(),
            AppView::SettingsAccount => self.list_state_settings_account.select_next(),
            AppView::SettingsLibrary => self.list_state_settings_library.select_next(),
        }
    }

    pub fn select_previous(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_previous(),
            AppView::Library => self.list_state_library.select_previous(),
            AppView::SearchBook => self.list_state_search_results.select_previous(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_previous(),
            AppView::Settings => self.list_state_settings.select_previous(),
            AppView::SettingsAccount => self.list_state_settings_account.select_previous(),
            AppView::SettingsLibrary => self.list_state_settings_library.select_previous(),
        }
    }

    pub fn select_first(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_first(),
            AppView::Library => self.list_state_library.select_first(),
            AppView::SearchBook => self.list_state_search_results.select_first(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_first(),
            AppView::Settings => self.list_state_settings.select_first(),
            AppView::SettingsAccount => self.list_state_settings_account.select_first(),
            AppView::SettingsLibrary => self.list_state_settings_library.select_first(),
        }
    }

    pub fn select_last(&mut self) {
        match self.view_state {
            AppView::Home => self.list_state_cnt_list.select_last(),
            AppView::Library => self.list_state_library.select_last(),
            AppView::SearchBook => self.list_state_search_results.select_last(),
            AppView::PodcastEpisode => self.list_state_pod_ep.select_last(),
            AppView::Settings => self.list_state_settings.select_last(),
            AppView::SettingsAccount => self.list_state_settings_account.select_last(),
            AppView::SettingsLibrary => self.list_state_settings_library.select_last(),
        }
    }

 }

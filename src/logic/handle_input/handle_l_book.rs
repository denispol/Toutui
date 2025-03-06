use crate::player::vlc::start_vlc::*;
use crate::player::vlc::fetch_vlc_data::*;
use crate::player::vlc::exec_nc::*;
use crate::utils::pop_up_message::*;
use crate::api::me::update_media_progress::*;
use crate::api::library_items::play_lib_item_or_pod::*;
use crate::api::sessions::sync_open_session::*;
use crate::api::sessions::close_open_session::*;
use std::io::stdout;
use log::{info, error};
use crate::db::crud::*;

pub async fn handle_l_book(
    token: Option<&String>,
    ids_library_items: Vec<String>,
    selected: Option<usize>,
    port: String,
    address_player: String,
    server_address: String,
    program: String,
    is_cvlc_term: String,
    username: String,
) {
  
    if let Some(index) = selected {
        if let Some(id) = ids_library_items.get(index) {
            if let Some(token) = token {
                if let Ok(info_item) = post_start_playback_session_book(Some(&token), id, server_address.clone()).await {
                    info!("[handle_l_book][post_start_playback_session_book] OK");
                    info!("[handle_l_book][post_start_playback_session_book] Item {} started at {}s", id, info_item[0]);

                    // converting current time
                    let current_time: u32 = info_item[0].parse().unwrap();

                    // insert variables in databse (`listening_session` table) for sync session when app is quit
                    let _ = insert_listening_session(
                        info_item[3].clone(), // id_session
                        id.to_string(), // id_item
                        current_time,  // current time
                        info_item[2].clone()); // total duration of the item
                        
                    // clone otherwise, these variable will  be consumed and not available anymore
                    // for use outside start_vlc spawn
                    let token_clone = token.clone();
                    let port_clone = port.clone();
                    let info_item_clone = info_item.clone() ;
                    let server_address_clone = server_address.clone() ;
                    let address_player_clone = address_player.clone() ;
                    // start_vlc is launched in a spawn to allow fetch_vlc_data to start at the same time
                    tokio::spawn(async move {
                        // this info! is not the most reliable to know is VLC is really launched
                        info!("[handle_l_book][start_vlc] VLC successfully launched");
                        start_vlc(
                            &info_item_clone[0], // current_time
                            &port_clone, // player port
                            address_player_clone, // player address
                            &info_item_clone[1], // content url 
                            Some(&token_clone), //token
                            info_item_clone[4].clone(), //title
                            info_item_clone[5].clone(), // subtitle
                            info_item_clone[6].clone(), //title
                            server_address_clone.clone(), // server address
                            program.clone(),
                        ).await;
                    });

                    if is_cvlc_term == "1" {
                        let port_clone = port.clone();
                        let address_player_clone = address_player.clone();
                        tokio::spawn(async move {
                            exec_nc(&port_clone, address_player_clone).await;
                        });
                    }

                    // clear loading message (from app.rs) when vlc is launched
                    let mut stdout = stdout(); 
                    let _ = clear_message(&mut stdout, 3);


                    // Important, sleep time to 1s minimum otherwise connection to vlc player will not have time to connect
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    // init var for decide to send 0 sec in sync session if player is in pause
                    // 3 sec is not very "pro" but it's because i'm sure for this first iteration
                    //   data_fetched_from_vlc will not be = to 3 (because a little delay is given
                    //   before sync progress, in my case 5 secs, others apps a little bit more)
                    //   futhermore, in the worst case, if data_fetched_from_vlc is equal ti 3 for
                    //   the first iteration, it will shift the progress sync to 5 secondes
                    let mut last_current_time: u32 = 3;
                    let mut progress_sync: u32 = 3;


                    loop {
                        match fetch_vlc_data(port.clone(), address_player.clone()).await {
                            Ok(Some(data_fetched_from_vlc)) => {
                                // println!("Fetched data: {}", data_fetched_from_vlc.to_string());

                                // update current_time in database (`listening_session` table)
                                let _ = update_current_time(data_fetched_from_vlc, info_item[3].as_str());

                                // Important, sleep time to 1s minimum, otherwise connection to vlc player will not have time to connect
                                // sleep time : every how many seconds the data will be sent to the server
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                // println!("last_curr: {}", last_current_time);
                                if data_fetched_from_vlc == last_current_time {
                                    progress_sync = 0; // the track is in pause
                                } else {
                                    progress_sync = 5; // need to be equal to tokio time sleep just above
                                }
                                last_current_time = data_fetched_from_vlc;
                                match fetch_vlc_is_playing(port.clone(), address_player.clone()).await {
                                    Ok(true) => {
                                        // the first datra fetched is sometimes 0 secondes, so we
                                        // want to be sure no send 0 secondes
                                        if Some(data_fetched_from_vlc) != Some(0) {
                                            let _ = sync_session(Some(&token), &info_item[3],Some(data_fetched_from_vlc), progress_sync, server_address.clone()).await;
                                            let _ = update_media_progress_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2], server_address.clone()).await;

                                        }
                                    },
                                    // `Ok(false)` means that the track is stopped but VLC still
                                    // open. Allow to track when the audio reached the end. And
                                    // differ from the case where the user just close VLC
                                    // during a playing (in this case we don't want to mark the
                                    // track as finished)
                                    Ok(false) => {
                                        let is_finised = true;

                                        // update is_finished in database (`listening_session` table)
                                        update_is_finished("1", info_item[3].as_str());
                                        
                                        let _ = close_session_without_send_prg_data(Some(&token), &info_item[3],  server_address.clone()).await;
                                        info!("[handle_l_book][Finished] Session successfully closed");
                                        let _ = update_media_progress2_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2], is_finised, server_address).await;
                                        info!("[handle_l_book][Finished] VLC stopped");
                                        info!("[handle_l_book][Finished] Item {} closed at {}s", id, data_fetched_from_vlc);
                                        let _ = update_is_loop_break("1", username.as_str());
                                        break; 
                                    },
                                    // `Err` means :  VLC is close (because if VLC is not playing
                                    // anymore an error is send by `fetch_vlc_is_playing`).
                                    // The track is not finished. VLC is just stopped by the user.
                                    // Differ from the case above where the track reched the end.
                                    Err(_) => {
                                        info!("[handle_l_book][Quit]");
                                        //TODO minor bug : be sure to close the session above
                                        // close session when VLC is quitted
                                        let _ = close_session_without_send_prg_data(Some(&token), &info_item[3],  server_address.clone()).await;
                                        info!("[handle_l_book][Quit] Session successfully closed");
                                        // send one last time media progress (bug to retrieve media
                                        // progress otherwise)
                                        let _ = update_media_progress_book(id, Some(&token), Some(data_fetched_from_vlc), &info_item[2], server_address).await;
                                        info!("[handle_l_book][Quit] VLC closed");
                                        info!("[handle_l_book][Quit] Item {} closed at {}s", id, data_fetched_from_vlc);
                                        //eprintln!("Error fetching play status: {}", e);
                                        //info!("[1] is_loop_break {}", loop_struct.is_loop_break);
                                        let _ = update_is_loop_break("1", username.as_str());
                                        break; 
                                    }
                                }

                            }
                            // when no data in fetched (generaly when VLC is launched and quit
                            // quickly) Indeed, in this case, data does not have enough time to be
                            // fetched
                            Ok(None) => {
                                info!("[handle_l_book][None]");
                                let _ = close_session_without_send_prg_data(Some(&token), &info_item[3],  server_address.clone()).await;
                                info!("[handle_l_book][None] Session successfully closed");
                                let _ = update_media_progress_book(id, Some(&token), Some(current_time), &info_item[2], server_address.clone()).await;
                                info!("[handle_l_book][None] VLC closed");
                                info!("[handle_l_book][None] Item {} closed at {}s", id, current_time);
                                //  loop_struct.is_loop_break = true;
                                //  info!("[2] is_loop_break {}", loop_struct.is_loop_break);
                                let _ = update_is_loop_break("1", username.as_str());
                                break; // Exit if no data available
                            }
                            Err(e) => {
                                error!("[handle_l_book][Err(e)]{}", e);
                             //   loop_struct.is_loop_break = true;
                             //   info!("[3] is_loop_break {}", loop_struct.is_loop_break);
                                break; // Exit on error
                            }
                        }
                    }
                } else {
                    error!("[handle_l_book] Failed to start playback session");
                    eprintln!("Failed to start playback session");
                }
            }
        }
    }
}


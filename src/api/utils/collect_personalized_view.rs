use crate::api::libraries::get_library_perso_view::Root;
use crate::utils::convert_seconds::*;

/// collect titles
pub async fn collect_titles_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut titles_cnt_list = Vec::new();  

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(title) = &metadata.title { 
                            titles_cnt_list.push(title.clone()); 
                        }
                    }
                }
            }
        }
    }

    titles_cnt_list  
}

/// collect author name 
pub async fn collect_auth_names_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut auth_names_cnt_list = Vec::new(); 

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(author_name) = &metadata.author_name { 
                            auth_names_cnt_list.push(author_name.clone()); 
                        }
                    }
                }
            }
        }
    }

    auth_names_cnt_list  
}

/// collect published year
pub async fn collect_pub_year_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut pub_year_cnt_list = Vec::new(); 

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(published_year) = &metadata.published_year { 
                            pub_year_cnt_list.push(published_year.clone()); 
                        }
                    }
                }
            }
        }
    }

    pub_year_cnt_list  
}

/// collect duration
pub async fn collect_duration_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut duration_cnt_list = Vec::new(); 

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                        if let Some(duration) = &media.duration { 
                            duration_cnt_list.push(duration.clone()); 
                        }
                }
            }
        }
    }

    let duration_cnt_list_converted = convert_seconds(duration_cnt_list);
    duration_cnt_list_converted
    
}

/// collect description
pub async fn collect_desc_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut desc_cnt_list = Vec::new(); 

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(description) = &metadata.description { 
                            desc_cnt_list.push(description.clone()); 
                        }
                    }
                }
            }
        }
    }

    desc_cnt_list  
}

/// collect ID of the library item
pub async fn collect_ids_cnt_list(continue_listening: &[Root]) -> Vec<String> {
    let mut ids_cnt_list = Vec::new();  

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                        if let Some(id) = &entity.id { 
                            ids_cnt_list.push(id.clone()); 
                        }
            }
        }
    }

    ids_cnt_list 
}

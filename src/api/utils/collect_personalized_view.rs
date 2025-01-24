use crate::api::libraries::get_library_perso_view::PersonalizedView;

/// collect titles
pub async fn collect_titles(continue_listening: &[PersonalizedView]) -> Vec<String> {
    let mut titles = Vec::new();  

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(title) = &metadata.title { 
                            titles.push(title.clone()); 
                        }
                    }
                }
            }
        }
    }

    titles  
}

/// collect author name 
pub async fn collect_author_name(continue_listening: &[PersonalizedView]) -> Vec<String> {
    let mut authors_names = Vec::new(); 

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                if let Some(media) = &entity.media {  
                    if let Some(metadata) = &media.metadata { 
                        if let Some(author_name) = &metadata.author_name { 
                            authors_names.push(author_name.clone()); 
                        }
                    }
                }
            }
        }
    }

    authors_names  
}

/// collect ID of the library item
pub async fn collect_ids_library_items(continue_listening: &[PersonalizedView]) -> Vec<String> {
    let mut ids_library_items = Vec::new();  

    for library in continue_listening {
        if let Some(entities) = &library.entities {
            for entity in entities {
                        if let Some(id) = &entity.id { 
                            ids_library_items.push(id.clone()); 
                        }
            }
        }
    }

    ids_library_items 
}

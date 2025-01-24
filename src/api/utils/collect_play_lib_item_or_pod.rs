
/// collect content url (link to stream the audio)
pub async fn collect_content_url(continue_listening: &[Root]) -> Vec<String> {
    let mut content: = Vec::new();

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


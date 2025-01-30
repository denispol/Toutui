use crate::api::libraries::get_all_books::Root;

/// collect titles
pub async fn collect_titles_library(library: &Root) -> Vec<String> {
    let mut titles_library = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                if let Some(metadata) = &media.metadata {
                    if let Some(title) = &metadata.title {
                        titles_library.push(title.clone());
                    }
                }
            }
        }
    }

    titles_library
}

/// collect ID of library items 
pub async fn collect_ids_library(library: &Root) -> Vec<String> {
    let mut ids_library = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(id) = &item.id {
                        ids_library.push(id.clone());
            }
        }
    }

    ids_library
}

/// collect author name
pub async fn collect_auth_names_library(library: &Root) -> Vec<String> {
    let mut auth_names_library = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                if let Some(metadata) = &media.metadata {
                    if let Some(author_name) = &metadata.author_name {
                        auth_names_library.push(author_name.clone());
                    }
                }
            }
        }
    }

    auth_names_library
}

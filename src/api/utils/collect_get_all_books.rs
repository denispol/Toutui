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


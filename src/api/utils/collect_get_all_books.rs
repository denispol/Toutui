use crate::api::libraries::get_all_books::Root;
use crate::utils::convert_seconds::*;

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

/// collect author name for book
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

/// collect author name for podcast
pub async fn collect_auth_names_library_pod(library: &Root) -> Vec<String> {
    let mut auth_names_library_pod = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                if let Some(metadata) = &media.metadata {
                    if let Some(author) = &metadata.author {
                        auth_names_library_pod.push(author.clone());
                    }
                }
            }
        }
    }

    auth_names_library_pod
}
/// collect published year
pub async fn collect_published_year_library(library: &Root) -> Vec<String> {
    let mut published_year_library = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                if let Some(metadata) = &media.metadata {
                    if let Some(pub_year) = &metadata.published_year {
                        published_year_library.push(pub_year.clone());
                    }
                }
            }
        }
    }

    published_year_library
}

/// collect published year
pub async fn collect_desc_library(library: &Root) -> Vec<String> {
    let mut desc_library = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                if let Some(metadata) = &media.metadata {
                    if let Some(desc) = &metadata.description {
                        desc_library.push(desc.clone());
                    }
                }
            }
        }
    }

    desc_library
}

/// collect published year
pub async fn collect_duration_library(library: &Root) -> Vec<String> {
    let mut duration = Vec::new();

    if let Some(results) = &library.results {
        for item in results {
            if let Some(media) = &item.media {
                    if let Some(dur) = &media.duration {
                        duration.push(dur.clone());
                }
            }
        }
    }

   let duration_library = convert_seconds(duration);
   duration_library
    
}

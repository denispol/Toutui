use crate::api::library_items::get_pod_ep::Root;

/// collect title podact episode
pub async fn collect_titles_pod_ep(item: &Root) -> Vec<String> {
    let mut titles_pod_ep = Vec::new();

    if let Some(media) = &item.media {
        if let Some(episodes) = &media.episodes {
            for episode in episodes {
                if let Some(title) = &episode.title {
                    titles_pod_ep.push(title.clone());
                }
            }
        }
    }

    titles_pod_ep
}

/// collect ID of podcast episode
pub async fn collect_ids_pod_ep(item: &Root) -> Vec<String> {
    let mut ids_pod_ep = Vec::new();

    if let Some(media) = &item.media {
        if let Some(episodes) = &media.episodes {
            for episode in episodes {
                if let Some(id) = &episode.id {
                    ids_pod_ep.push(id.clone());
                }
            }
        }
    }

    ids_pod_ep
}



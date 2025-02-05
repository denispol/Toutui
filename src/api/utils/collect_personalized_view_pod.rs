use crate::api::libraries::get_library_perso_view_pod::Root;

/// collect id pod for continue listening
pub async fn collect_ids_pod_cnt_list(roots: &[Root]) -> Vec<String> {
    let mut ids_pod_cnt_list = Vec::new();

    for root in roots {
        if let Some(entities) = &root.entities {
            for entity in entities {
                if let Some(recent_episode) = &entity.recent_episode {
                    if let Some(library_item_id) = recent_episode.library_item_id.clone() {
                        ids_pod_cnt_list.push(library_item_id);
                    }
                }
            }
        }
    }

 ids_pod_cnt_list
}

/// collect id episode pod for continue listening
pub async fn collect_ids_ep_pod_cnt_list(roots: &[Root]) -> Vec<String> {
    let mut ids_ep_pod_cnt_list = Vec::new();

    for root in roots {
        if let Some(entities) = &root.entities {
            for entity in entities {
                if let Some(recent_episode) = &entity.recent_episode {
                    if let Some(id) = recent_episode.id.clone() {
                        ids_ep_pod_cnt_list.push(id);
                    }
                }
            }
        }
    }

 ids_ep_pod_cnt_list
}

/// collect titles pod for continue listening
pub async fn collect_titles_cnt_list_pod(roots: &[Root]) -> Vec<String> {
    let mut titles_cnt_list = Vec::new();


    for root in roots {
        if let Some(entities) = &root.entities {
            for entity in entities {
                if let Some(recent_episode) = &entity.recent_episode {
                    if let Some(title) = recent_episode.title.clone() {
                        titles_cnt_list.push(title);
                    }
                }
            }
        }
    }

    titles_cnt_list
}

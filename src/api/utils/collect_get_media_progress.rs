use crate::api::me::get_media_progress::Root;
use crate::utils::convert_seconds::*;

pub async fn collect_progress_percentage_book(root: &Root) -> String {
    format!("{}", (root.progress * 100.0).round() as i64) 
}

pub async fn collect_is_finished_book(item: &Root) -> String {
    if item.is_finished {
        "Finished".to_string() 
    } else {
        "Not finished".to_string() 
    }
}

pub async fn collect_remaining_time(item: &Root) -> String {
    let remaining_time = item.duration - item.current_time; 
    let total_minutes = (remaining_time / 60.0).round() as i64;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    if hours == 0 {
        format!("{}m left", minutes)
    } else if minutes == 0 {
        format!("{}h left", hours)
    } else {
        format!("{}h{}m left", hours, minutes)
    }

}



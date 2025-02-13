pub fn convert_seconds(vec_seconds: Vec<f64>) -> Vec<String> {
    vec_seconds.iter()
        .map(|&s| {
            let total_minutes = (s / 60.0).round() as i64;
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            if hours == 0 {
                format!("{}m", minutes)
            } else if minutes == 0 {
                format!("{}h", hours)
            } else {
                format!("{}h{}m", hours, minutes)
            }
        })
        .collect()
}

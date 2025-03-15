pub fn get_dynamic_text() -> String {
    // Par exemple, générer un texte basé sur l'heure actuelle
    let current_time = chrono::Local::now().to_rfc3339();
    format!("{}", current_time)
}


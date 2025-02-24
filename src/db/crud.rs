use rusqlite::{params, Connection, Result};
use crate::db::database_struct::User;
use crate::utils::pop_up_message::*;
use std::io::stdout;
use log::{info, error};
use std::path::PathBuf;

// Delete an user
pub fn delete_user(username: &str) -> Result<()> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let message = format!("User '{}' deleted. Please restart the app to apply the changes.", &username);
    let err_message = "Error connecting to the database.";
    if let Ok(conn) = Connection::open(db_path) {

        let rows_deleted = conn.execute(
            "DELETE FROM users WHERE username = ?1",
            params![username],
        )?;

        if rows_deleted > 0 {
            let mut stdout = stdout();
            let _ = pop_message(&mut stdout, 3, message.as_str());
            info!("[delete_user] User deleted.");
        } else {
            //println!("No user found with this username '{}'.", username);
        }
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[delete user] {}", err_message);
    }

    Ok(())
}

// Update id_previous_listening_session
pub fn update_id_prev_list_session(id: &str, username: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let err_message = "Error connecting to the database.";

    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE users SET id_previous_listening_session = ?1 WHERE username = ?2",
            params![id, username],
        )?;
    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_id_prev_list_session] {}", err_message);
    }

    Ok(())
}


// get id_previous_listening_session
pub fn get_id_prev_list_session(username: &str) -> String {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(_) => return String::from("Error: unable open database"),
    };

    let mut stmt = match conn.prepare("SELECT id_previous_listening_session FROM users WHERE username = ?1") {
        Ok(s) => s,
        Err(_) => return String::from("Error to prepare reqwest"),
    };

    match stmt.query_row(params![username], |row| row.get::<_, String>(0)) {
        Ok(id) => id,
        Err(_) => String::from("No db found"),
    }
}

// Update id_selected_lib
pub fn update_id_selected_lib(id_selected_lib: &str, username: &str) -> Result<()> {

    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let message = "The library has been updated. Please refresh the app to apply the changes.";
    let err_message = "Error connecting to the database.";
    if let Ok(conn) = Connection::open(db_path) {

        conn.execute(
            "UPDATE users SET id_selected_lib = ?1 WHERE username = ?2",
            params![id_selected_lib, username],
        )?;
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, message);
        info!("[update_id_selected_lib] The library has been updated");

    } else {
        let mut stdout = stdout();
        let _ = pop_message(&mut stdout, 3, err_message);
        error!("[update_id_selected_lib] {}", err_message);
    }

    Ok(())
}

// update default user 
pub fn update_default_user(conn: &Connection, username: &str) -> Result<()> {
    // Mark all user as 0 by default
    conn.execute(
        "UPDATE users SET is_default_usr = 0",
        [],
    )?;

    // Put the desired user as default
    conn.execute(
        "UPDATE users SET is_default_usr = 1 WHERE username = ?1",
        params![username],
    )?;

    Ok(())
}

// Insert user in database
pub fn db_insert_usr(users : &Vec<User>)  -> Result<()> {   
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = Connection::open(db_path)?;
    for user in users {
        conn.execute(
            "INSERT OR REPLACE INTO users (username, server_address, token, is_default_usr, name_selected_lib, id_selected_lib, id_previous_listening_session) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
            user.username,
            user.server_address,
            user.token,
            if user.is_default_usr { 1 } else { 0 },
            user.name_selected_lib,
            user.id_selected_lib,
            user.id_previous_listening_session
            ],
        )?;
    }
    Ok(())
}


// Select default user
pub fn select_default_usr() -> Result<Vec<String>> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT username, server_address, token, is_default_usr, name_selected_lib, id_selected_lib, id_previous_listening_session
         FROM users WHERE is_default_usr = 1 LIMIT 1"
    )?;


    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            server_address: row.get(1)?,
            token: row.get(2)?,
            is_default_usr: row.get::<_, i32>(3)? != 0,  // Convertir 0/1 en bool
            name_selected_lib: row.get(4)?,
            id_selected_lib: row.get(5)?,
            id_previous_listening_session: row.get(6)?,
        })
    })?;

    let mut result = Vec::new();

    for user in user_iter {
        match user {
            Ok(user) => {
                result.push(user.username);
                result.push(user.server_address);
                result.push(user.token);
                result.push(user.is_default_usr.to_string());
                result.push(user.name_selected_lib);
                result.push(user.id_selected_lib);
                result.push(user.id_previous_listening_session);
            }
            Err(e) => {
                println!("Error occurred: {}", e);
                //return Err(rusqlite::Error::FromSqlConversionFailure(0, "Failed to map user".to_string()));
            }
        }
    }

    if result.is_empty() {
        //println!("No default user found.");
    }

    Ok(result)  
}

// Init db and table if not exist
pub fn init_db() -> Result<()> {
    let mut db_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push("toutui/db.sqlite3");

    // Open or create db
    let conn = Connection::open(db_path)?;

    //Create a table if there is none 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                username TEXT PRIMARY KEY,
                server_address TEXT NOT NULL,
                token TEXT NOT NULL,
                is_default_usr INTEGER NOT NULL DEFAULT 0,
                name_selected_lib TEXT NOT NULL,
                id_selected_lib TEXT NOT NULL,
                id_previous_listening_session TEXT NOT NULL
            )",
        [],
    )?;


    Ok(())
}



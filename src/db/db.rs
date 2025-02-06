use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    server_adress: String,
    username: String,
    password: String, // ⚠️ Chiffre le mot de passe avant de le stocker !
    is_default_usr: bool,
    name_selected_lib: String,
    id_selected_lib: String,
}

pub fn db() -> Result<()> {
    // Ouvre ou crée une base de données SQLite
    let conn = Connection::open("db.sqlite3")?;

    // Crée une table pour les utilisateurs si elle n'existe pas
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            server_adress TEXT NOT NULL,
            password TEXT NOT NULL,
            is_default_usr INTEGER NOT NULL DEFAULT 0,
            name_selected_lib TEXT NOT NULL,
            id_selected_lib TEXT NOT NULL
        )",
        [],
    )?;

    // Crée un vecteur d'utilisateurs
    let users = vec![
        User {
            server_adress: "https://nuagemagique.duckdns.org".to_string(),
            username: "luc".to_string(),
            password: "acac".to_string(),
            is_default_usr: true,
            name_selected_lib: "LeNuageMagique".to_string(),
            id_selected_lib: "5d80300e-e228-402e-9b6e-1356ff1f4243".to_string(),
        },
        User {
            server_adress: "https://example.com".to_string(),
            username: "alice".to_string(),
            password: "securepassword".to_string(),
            is_default_usr: false,
            name_selected_lib: "Library2".to_string(),
            id_selected_lib: "12345678-aaaa-bbbb-cccc-1356ff1f4243".to_string(),
        },
    ];

    // Insère les utilisateurs dans la base de données
    for user in users {
        conn.execute(
            "INSERT OR REPLACE INTO users (username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                user.username,
                user.server_adress,
                user.password,
                if user.is_default_usr { 1 } else { 0 },
                user.name_selected_lib,
                user.id_selected_lib,
            ],
        )?;
    }

    // Récupérer un utilisateur spécifique (par exemple "luc")
    let username = "luc";
    let mut stmt = conn.prepare("SELECT username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib FROM users WHERE username = ?1")?;
    let user_iter = stmt.query_map(params![username], |row| {
        Ok(User {
            username: row.get(0)?,
            server_adress: row.get(1)?,
            password: row.get(2)?,
            is_default_usr: row.get::<_, i32>(3)? != 0,  // Forcer le type i32 pour is_default_usr
            name_selected_lib: row.get(4)?,
            id_selected_lib: row.get(5)?,
        })
    })?;

    for user in user_iter {
        match user {
            Ok(user) => println!("User found: {:?}", user),
            Err(e) => println!("Error occurred: {}", e),
        }
    }

    // Afficher tous les utilisateurs
    let mut stmt = conn.prepare("SELECT username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            server_adress: row.get(1)?,
            password: row.get(2)?,
            is_default_usr: row.get::<_, i32>(3)? != 0,  // Forcer le type i32 pour is_default_usr
            name_selected_lib: row.get(4)?,
            id_selected_lib: row.get(5)?,
        })
    })?;

    for user in user_iter {
        match user {
            Ok(user) => println!("User: {:?}", user),
            Err(e) => println!("Error occurred: {}", e),
        }
    }

    Ok(())
}



use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use crate::db::database_struct::User;

// Update id_selected_lib
pub fn update_id_selected_lib(conn: &Connection, id_selected_lib: &str, username: &str) -> Result<()> {

    //  SQL request
    conn.execute(
        "UPDATE users SET id_selected_lib = ?1 WHERE username = ?2",
        params![id_selected_lib, username],
    )?;

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
    let conn = Connection::open("db/db.sqlite3")?;
    for user in users {
        conn.execute(
            "INSERT OR REPLACE INTO users (username, server_adress, password, token, is_default_usr, name_selected_lib, id_selected_lib) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
            user.username,
            user.server_adress,
            user.password,
            user.token,
            if user.is_default_usr { 1 } else { 0 },
            user.name_selected_lib,
            user.id_selected_lib,
            ],
        )?;
    }
    Ok(())
}


// Select default user
pub fn select_default_usr() -> Result<Vec<String>> {
    let conn = Connection::open("db/db.sqlite3")?;

    // Prépare la requête SQL
    let mut stmt = conn.prepare(
        "SELECT username, server_adress, password, token, is_default_usr, name_selected_lib, id_selected_lib
         FROM users WHERE is_default_usr = 1 LIMIT 1"
    )?;

    // Effectue la requête et mappe les résultats
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            server_adress: row.get(1)?,
            password: row.get(2)?,
            token: row.get(3)?,
            is_default_usr: row.get::<_, i32>(4)? != 0,  // Convertir 0/1 en bool
            name_selected_lib: row.get(5)?,
            id_selected_lib: row.get(6)?,
        })
    })?;

    // Créer un vecteur pour stocker les résultats
    let mut result = Vec::new();

    // Boucle sur les résultats et collecter les informations
    for user in user_iter {
        match user {
            Ok(user) => {
                // Nous extrayons les informations sous forme de String (par exemple, username)
                result.push(user.username);
                result.push(user.server_adress);
                result.push(user.password);
                result.push(user.token);
                result.push(user.is_default_usr.to_string());
                result.push(user.name_selected_lib);
                result.push(user.id_selected_lib);
                //println!("Default user found: {:?}", &user);  // Affichage pour le débogage
            }
            Err(e) => {
                // Gérer l'erreur et retourner un Result avec l'erreur
                println!("Error occurred: {}", e);
                //return Err(rusqlite::Error::FromSqlConversionFailure(0, "Failed to map user".to_string()));
            }
        }
    }

    // Si aucun utilisateur trouvé, retourne un vecteur vide
    if result.is_empty() {
        println!("No default user found.");
    }

    Ok(result)  // Retourne le vecteur des utilisateurs par défaut
}

// General 
pub fn db() -> Result<()> {
    // Ouvre ou crée une base de données SQLite
    let conn = Connection::open("db/db.sqlite3")?;
  //conn.execute("DROP TABLE IF EXISTS users", [])?;
//     Crée une table pour les utilisateurs si elle n'existe pas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                username TEXT PRIMARY KEY,
                server_adress TEXT NOT NULL,
                password TEXT NOT NULL,
                token TEXT NOT NULL,
                is_default_usr INTEGER NOT NULL DEFAULT 0,
                name_selected_lib TEXT NOT NULL,
                id_selected_lib TEXT NOT NULL
            )",
        [],
    )?;


//    // Récupérer un utilisateur spécifique (par exemple "luc")
//    let username = "luc";
//    let mut stmt = conn.prepare("SELECT username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib FROM users WHERE username = ?1")?;
//    let user_iter = stmt.query_map(params![username], |row| {
//        Ok(User {
//            username: row.get(0)?,
//            server_adress: row.get(1)?,
//            password: row.get(2)?,
//            is_default_usr: row.get::<_, i32>(3)? != 0,  // Forcer le type i32 pour is_default_usr
//            name_selected_lib: row.get(4)?,
//            id_selected_lib: row.get(5)?,
//        })
//    })?;
//
//    for user in user_iter {
//        match user {
//            Ok(user) => println!("User found: {:?}", user),
//            Err(e) => println!("Error occurred: {}", e),
//        }
//    }
//
//    // Afficher tous les utilisateurs
//    let mut stmt = conn.prepare("SELECT username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib FROM users")?;
//    let user_iter = stmt.query_map([], |row| {
//        Ok(User {
//            username: row.get(0)?,
//            server_adress: row.get(1)?,
//            password: row.get(2)?,
//            token: row.get(3)?,
//            is_default_usr: row.get::<_, i32>(4)? != 0,  // Forcer le type i32 pour is_default_usr
//            name_selected_lib: row.get(5)?,
//            id_selected_lib: row.get(6)?,
//        })
//    })?;
//
//    for user in user_iter {
//        match user {
//            Ok(user) => println!("User: {:?}", user),
//            Err(e) => println!("Error occurred: {}", e),
//        }
//    }
//
//
    Ok(())
}



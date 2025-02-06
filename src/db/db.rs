use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use crate::app::User;


    // Insère les utilisateurs dans la base de données
pub fn db_insert_usr(users : &Vec<User>)  -> Result<()> {   
    let conn = Connection::open("db.sqlite3")?;
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
    Ok(())
}


    // select default user
pub fn select_default_usr() -> Result<Vec<String>> {
    let conn = Connection::open("db.sqlite3")?;
    
    // Prépare la requête SQL
    let mut stmt = conn.prepare(
        "SELECT username, server_adress, password, is_default_usr, name_selected_lib, id_selected_lib
         FROM users WHERE is_default_usr = 1 LIMIT 1"
    )?;

    // Effectue la requête et mappe les résultats
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            server_adress: row.get(1)?,
            password: row.get(2)?,
            is_default_usr: row.get::<_, i32>(3)? != 0,  // Convertir 0/1 en bool
            name_selected_lib: row.get(4)?,
            id_selected_lib: row.get(5)?,
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
pub fn db() -> Result<()> {
    // Ouvre ou crée une base de données SQLite
    let conn = Connection::open("db.sqlite3")?;

//    // Crée une table pour les utilisateurs si elle n'existe pas
//    conn.execute(
//        "CREATE TABLE IF NOT EXISTS users (
//            username TEXT PRIMARY KEY,
//            server_adress TEXT NOT NULL,
//            password TEXT NOT NULL,
//            is_default_usr INTEGER NOT NULL DEFAULT 0,
//            name_selected_lib TEXT NOT NULL,
//            id_selected_lib TEXT NOT NULL
//        )",
//        [],
//    )?;


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



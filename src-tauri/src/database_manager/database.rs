use rusqlite::{Connection, Result};
use std::{sync::{Arc, Mutex}, fs};
use tauri::{command, Manager, State};
use std::path::{Path, PathBuf};

use super::entities;
use super::loot_groups;

pub struct Database {
    pub conn: Arc<Mutex<Connection>>
}


impl Database {
    pub fn load_or_create_database(path: &str, db_path: &str) -> Result<Self> {
        let dir_path = Path::new(path).join("data");
        let full_path = dir_path.join(db_path);


        if !dir_path.exists() {
            fs::create_dir_all(&dir_path);
        }

        let conn = Connection::open(&full_path)?;

        // if Path::new(&full_path).exists() {
        //     println!("Database already exists: {}", full_path);
        //     return Ok(Database {
        //         conn: Arc::new(Mutex::new(conn)),
        //     });
        // }

        entities::create_entity_table(&conn);
        loot_groups::create_loot_group_table(&conn);
            // conn.execute(
            //     "CREATE TABLE IF NOT EXISTS entities (
            //         entity_id INTEGER PRIMARY KEY AUTOINCREMENT,
            //         name TEXT NOT NULL,
            //         kind TEXT NULL,
            //         initiative INTEGER NULL,
            //         health INTEGER NULL
            //         armor_class INTEGER NULL,
            //         speed_ground INTEGER NULL,
            //         speed_air INTEGER NULL,
            //         speed_water INTEGER NULL,
            //         strength INTEGER NULL,
            //         dexterity INTEGER NULL,
            //         constitution INTEGER NULL,
            //         intelligence INTEGER NULL,
            //         wisdom INTEGER NULL,
            //         charisma INTEGER NULL,`
            //         skills TEXT NULL,
            //         gear TEXT NULL,
            //         challenge_rating INTEGER NULL,
            //         xp INTEGER NULL,
            //         traits TEXT NULL,
            //         actions TEXT NULL,
            //         bonus_actions TEXT NULL,
            //     )",
            //     [],
            // )?;

            conn.execute( //TODO Create table for Entity Languages
                "CREATE TABLE IF NOT EXISTS entity_languages (
                    entity_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    kind TEXT NULL,
                    initiative INTEGER NULL,
                    health INTEGER NULL,
                    armor_class INTEGER NULL,
                    speed_ground INTEGER NULL,
                    speed_air INTEGER NULL,
                    speed_water INTEGER NULL,
                    strength INTEGER NULL,
                    dexterity INTEGER NULL,
                    constitution INTEGER NULL,
                    intelligence INTEGER NULL,
                    wisdom INTEGER NULL,
                    charisma INTEGER NULL,
                    skills TEXT NULL,
                    gear TEXT NULL,
                    challenge_rating INTEGER NULL,
                    xp INTEGER NULL,
                    traits TEXT NULL,
                    actions TEXT NULL,
                    bonus_actions TEXT NULL
                )",
                [],
            )?;
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS characters (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT NULL
                )",
                [],
            )?;
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT NULL
                )",
                [],
            )?;
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS entity_items (
                    entity_id INT,
                    item_id INT,
                    PRIMARY KEY (entity_id, item_id),
                    FOREIGN KEY (entity_id) REFERENCES entities(entity_id) ON DELETE CASCADE,
                    FOREIGN KEY (item_id) REFERENCES items(item_id) ON DELETE CASCADE
                );",
                [],
            )?;
        
        
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS places (
                    place_id INT PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    description TEXT
                );",
                [],
            )?;
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS encounters (
                        encounter_id INT PRIMARY KEY,
                        place_id INT,
                        name VARCHAR(255),
                        description TEXT,
                        FOREIGN KEY (place_id) REFERENCES places(place_id) ON DELETE CASCADE
                    );",
                [],
            )?;
        
        
            conn.execute(
                "CREATE TABLE IF NOT EXISTS encounter_entities (
                        encounter_id INT,
                        entity_id INT,
                        PRIMARY KEY (encounter_id, entity_id),
                        FOREIGN KEY (encounter_id) REFERENCES encounters(encounter_id) ON DELETE CASCADE,
                        FOREIGN KEY (entity_id) REFERENCES entities(entity_id) ON DELETE CASCADE
                    );",
                [],
            )?;

        
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        let conn = self.conn.lock().unwrap();
        Ok(conn)
    }
    
    
    pub fn add_entity(&self, name: &str, description: Option<&str>) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute("INSERT INTO enemies (name, description) VALUES (?1, ?2)", rusqlite::params![name, description])?;
        Ok(())
    }

    // Add a new character
    pub fn add_character(&self, name: &str, description: Option<&str>) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO characters (name, description) VALUES (?1, ?2)",
            rusqlite::params![name, description],
        )?;
        Ok(())
    }

    // Add a new item
    pub fn add_item(&self, name: &str, description: Option<&str>) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO items (name, description) VALUES (?1, ?2)",
            rusqlite::params![name, description],
        )?;
        Ok(())
    }

    // Add a new place
    pub fn add_place(&self, place_id: i32, name: &str, description: Option<&str>) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO places (place_id, name, description) VALUES (?1, ?2, ?3)",
            rusqlite::params![place_id, name, description],
        )?;
        Ok(())
    }

    // Add a new encounter
    pub fn add_encounter(&self, encounter_id: i32, place_id: i32, name: &str, description: Option<&str>) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO encounters (encounter_id, place_id, name, description) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![encounter_id, place_id, name, description],
        )?;
        Ok(())
    }

    // Add an entity to an encounter (encounter_entities table)
    pub fn add_encounter_entity(&self, encounter_id: i32, entity_id: i32) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO encounter_entities (encounter_id, entity_id) VALUES (?1, ?2)",
            rusqlite::params![encounter_id, entity_id],
        )?;
        Ok(())
    }

    // Add an item to an entity (entity_items table)
    pub fn add_entity_item(&self, entity_id: i32, item_id: i32) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO entity_items (entity_id, item_id) VALUES (?1, ?2)",
            rusqlite::params![entity_id, item_id],
        )?;
        Ok(())
    }

}


// These functions now act as Tauri commands
#[command]
pub fn add_entity(db: State<'_, Database>, name: String, description: Option<String>) -> Result<(), String> {
    db.add_entity(&name, description.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn add_character(db: State<'_, Database>, name: String, description: Option<String>) -> Result<(), String> {
    db.add_character(&name, description.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn add_item(db: State<'_, Database>, name: String, description: Option<String>) -> Result<(), String> {
    db.add_item(&name, description.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn add_place(db: State<'_, Database>, place_id: i32, name: String, description: Option<String>) -> Result<(), String> {
    db.add_place(place_id, &name, description.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn add_encounter(db: State<'_, Database>, encounter_id: i32, place_id: i32, name: String, description: Option<String>) -> Result<(), String> {
    db.add_encounter(encounter_id, place_id, &name, description.as_deref()).map_err(|e| e.to_string())
}

#[command]
pub fn add_encounter_entity(db: State<'_, Database>, encounter_id: i32, entity_id: i32) -> Result<(), String> {
    db.add_encounter_entity(encounter_id, entity_id).map_err(|e| e.to_string())
}

#[command]
pub fn add_entity_item(db: State<'_, Database>, entity_id: i32, item_id: i32) -> Result<(), String> {
    db.add_entity_item(entity_id, item_id).map_err(|e| e.to_string())
}
//{
 //   db.add_encounter_entity(encounter_id, entity_id).map_err(|e| e.to_string())
//}
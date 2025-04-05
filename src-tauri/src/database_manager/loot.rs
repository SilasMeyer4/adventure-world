use tauri::State;
use serde::{Serialize, Deserialize};
use crate::database_manager::database::Database;
use rusqlite::{Connection, Result};


#[derive(Debug, Deserialize, Serialize)]
pub struct LootGroup_Loot {
    lootgroup_id: u32,
    item_id: u32,
    rate_value: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity_Loot {
    entity_id: u32,
    item_id: u32,
    rate_value: u32,
}



pub fn create_loot_group_loot_table(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS loot_group_loot (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            loot_group_id TEXT NOT NULL,
            item_id INTEGER NOT NULL,
            rate_value INTEGER NOT NULL,
            FOREIGN KEY (loot_group_id) REFERENCES loot_groups(id) ON DELETE CASCADE,
            FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
        )"
    ).expect("Was not able to create loot_group_loot table");

    Ok(())
}

pub fn create_entity_loot_table(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS entity_loot (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entity_id TEXT NOT NULL,
            item_id INTEGER NOT NULL,
            rate_value INTEGER NOT NULL,
            FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
            FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
        )"
    ).expect("Was not able to create entity_loot table");

    Ok(())
}

#[tauri::command]
pub fn add_loot_to_group(db: State<'_, Database>, loot: LootGroup_Loot) -> Result<(), String> {
    println!("Adding loot group: {:?}", loot);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    println!("lcoked database");
    conn.execute(
        "INSERT INTO loot_group_loot (
            loot_group_id, item_id, rate_value
        ) VALUES (
            ?, ?, ?
        )",
        rusqlite::params![
            loot.loot_group_id,
            loot.item_id,
            loot.rate_value,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())

}


#[tauri::command]
pub fn add_loot_to_entity(db: State<'_, Database>, loot: Entity_Loot) -> Result<(), String> {
    println!("Adding loot group: {:?}", loot);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    println!("lcoked database");
    conn.execute(
        "INSERT INTO loot_group_loot (
            loot_group_id, item_id, rate_value
        ) VALUES (
            ?, ?, ?
        )",
        rusqlite::params![
            loot.entity_id,
            loot.item_id,
            loot.rate_value,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())

}
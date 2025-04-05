use tauri::State;
use serde::{Serialize, Deserialize};
use crate::database_manager::database::Database;
use rusqlite::{Connection, Result};


#[derive(Debug, Deserialize, Serialize)]
pub struct LootGroup {
    name: String,
    probability: Option<u8>,
    amount: Option<u32>,
}



pub fn create_loot_group_table(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS loot_group (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            probability INTEGER NOT NULL
        )"
    ).expect("Was not able to create loot_group table");

    Ok(())
}

#[tauri::command]
pub fn add_loot_group(db: State<'_, Database>, lootgroup: LootGroup) -> Result<(), String> {
    println!("Adding loot group: {:?}", lootgroup);

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    println!("lcoked database");
    conn.execute(
        "INSERT INTO loot_group (
            name, probability, amount
        ) VALUES (
            ?, ?, ?
        )",
        rusqlite::params![
            lootgroup.name,
            lootgroup.probability,
            lootgroup.amount,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(())

}

#[tauri::command]
pub fn get_loot_group_by_id(db: State<'_, Database>, id: u32) -> Result<LootGroup, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string()).expect("Error locking");
    
    // Prepare the query to select the loot group by its id
    let mut stmt = conn
        .prepare("SELECT id, name, probability, amount FROM loot_group WHERE id = ?")
        .map_err(|e| e.to_string())?;

    // Execute the query and fetch the result
    let loot_group = stmt
        .query_row([id], |row| {
            Ok(LootGroup {
                name: row.get(1)?,
                probability: row.get(2)?,
                amount: row.get(3)?,
            })
        })
        .map_err(|e| format!("Failed to fetch loot group: {}", e))?;

    Ok(loot_group)
}


#[tauri::command]
pub fn search_loot_group_by_name(db: State<'_, Database>, name: String) -> Result<Vec<LootGroup>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string()).expect("Error locking database");

    // Prepare the query to select loot groups by name using the LIKE operator
    let mut stmt = conn
        .prepare("SELECT id, name, probability, amount FROM loot_group WHERE name LIKE ?")
        .map_err(|e| e.to_string())?;

    // Execute the query and fetch all matching loot groups
    let loot_groups = stmt
        .query_map([format!("%{}%", name)], |row| {
            Ok(LootGroup {
                name: row.get(1)?,
                probability: row.get(2)?,
                amount: row.get(3)?,
            })
        })
        .map_err(|e| format!("Failed to fetch loot groups: {}", e))?;

    let result: Vec<LootGroup> = loot_groups.collect::<Result<_, _>>().map_err(|e| e.to_string())?;

    Ok(result)
}
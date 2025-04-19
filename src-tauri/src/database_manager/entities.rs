use crate::database_manager::database::Database;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct Monster {
    pub name: String,
    pub source: Option<String>,
    pub page: Option<i32>,
    pub size: Vec<String>,
    #[serde(rename = "type")]
    pub monster_type: String,
    pub alignment: Vec<String>,
    pub ac: Vec<i32>,
    pub hp: Hp,
    pub speed: Speed,
    pub str_: i32, // Rename to avoid keyword conflict
    pub dex: i32,
    pub con: i32,
    pub int_: i32,
    pub wis: i32,
    pub cha: i32,
    pub passive: Option<i32>,
    pub cr: String,

    pub skill: Option<serde_json::Value>,
    pub trait_: Option<serde_json::Value>,
    pub action: Option<serde_json::Value>,

    pub trait_tags: Option<Vec<String>>,
    pub sense_tags: Option<Vec<String>>,
    pub damage_tags: Option<Vec<String>>,
    pub misc_tags: Option<Vec<String>>,

    pub hasToken: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Hp {
    pub average: i32,
    pub formula: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Speed {
    pub walk: Option<i32>,
    pub burrow: Option<i32>,
    pub climb: Option<i32>,
    pub fly: Option<i32>,
    pub swim: Option<i32>,
}

pub fn create_entity_table(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS entities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            source TEXT,
            page INTEGER,
            size TEXT,
            type TEXT,
            alignment TEXT,
            ac INTEGER,
            hp_avg INTEGER,
            hp_formula TEXT,
            speed_walk INTEGER,
            speed_burrow INTEGER,
            speed_climb INTEGER,
            speed_fly INTEGER,
            speed_swim INTEGER,
            str INTEGER,
            dex INTEGER,
            con INTEGER,
            int INTEGER,
            wis INTEGER,
            cha INTEGER,
            passive INTEGER,
            cr TEXT,
            skill_json TEXT,
            traits_json TEXT,
            actions_json TEXT,
            trait_tags TEXT,
            sense_tags TEXT,
            damage_tags TEXT,
            misc_tags TEXT,
            has_token BOOLEAN
        );
        ",
    )
    .expect("Was nota able to create entity table");

    Ok(())
}

#[tauri::command]
pub fn insert_monster(db: State<'_, Database>, monster: Monster) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO entities (
            name, source, page, size, type, alignment,
            ac, hp_avg, hp_formula,
            speed_walk, speed_burrow, speed_climb, speed_fly, speed_swim,
            str, dex, con, int, wis, cha,
            passive, cr,
            skill_json, traits_json, actions_json,
            trait_tags, sense_tags, damage_tags, misc_tags, has_token
        ) VALUES (
            ?, ?, ?, ?, ?, ?,
            ?, ?, ?,
            ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?, ?,
            ?, ?,
            ?, ?, ?,
            ?, ?, ?, ?, ?
        )",
        rusqlite::params![
            monster.name,
            monster.source,
            monster.page,
            monster.size.join(","),
            monster.monster_type,
            monster.alignment.join(","),
            monster.ac.get(0).copied().unwrap_or(10),
            monster.hp.average,
            monster.hp.formula,
            monster.speed.walk.unwrap_or(0),
            monster.speed.burrow.unwrap_or(0),
            monster.speed.climb.unwrap_or(0),
            monster.speed.fly.unwrap_or(0),
            monster.speed.swim.unwrap_or(0),
            monster.str_,
            monster.dex,
            monster.con,
            monster.int_,
            monster.wis,
            monster.cha,
            monster.passive,
            monster.cr,
            monster.skill.map(|v| v.to_string()),
            monster.trait_.map(|v| v.to_string()),
            monster.action.map(|v| v.to_string()),
            monster.trait_tags.map(|v| v.join(",")),
            monster.sense_tags.map(|v| v.join(",")),
            monster.damage_tags.map(|v| v.join(",")),
            monster.misc_tags.map(|v| v.join(",")),
            monster.hasToken.unwrap_or(false),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

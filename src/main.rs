use std::collections::HashMap;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("dogs.db")?;

    let mut dog_colors = HashMap::new();
    dog_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    dog_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    conn.execute(
        "create table if not exists dog_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists dogs (
             id integer primary key,
             name text not null,
             color_id integer not null references dog_colors(id)
         )",
        [],
    )?;

    Ok(())
}

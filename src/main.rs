use std::collections::HashMap;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Dog {
    name: String,
    color: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("dogs.db")?;

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

    let mut dog_colors = HashMap::new();
    dog_colors.insert(String::from("Grey"), vec!["Ruger", "Rocko"]);
    dog_colors.insert(String::from("Black"), vec!["Nexus", "Freezo"]);

    conn.execute("delete from dogs", [])?;
    conn.execute("delete from dog_colors", [])?;
    for (color, dognames) in &dog_colors {
        conn.execute(
            "INSERT INTO dog_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for dog in dognames {
            conn.execute(
                "INSERT INTO dogs (name, color_id) values (?1, ?2)",
                &[&dog.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from dogs c
         INNER JOIN dog_colors cc
         ON cc.id = c.color_id;",
    )?;

    let dogs = stmt.query_map([], |row| {
        Ok(Dog {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for dog in dogs {
        println!("Found dog {:?}", dog);
    }

    let mut conn = Connection::open("dogs.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    println!("Error[{}]: {:?}", res.is_err(), res);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from dogs", [])?;
    tx.execute("delete from dog_colors", [])?;
    tx.execute("insert into dog_colors (name) values (?1)", &[&"purple"])?;
    tx.execute("insert into dog_colors (name) values (?1)", &[&"green"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from dog_colors", [])?;
    tx.execute("insert into dog_colors (name) values (?1)", &[&"purple"])?;
    tx.execute("insert into dog_colors (name) values (?1)", &[&"green"])?;
    tx.execute("insert into dog_colors (name) values (?1)", &[&"purple"])?;

    tx.commit()
}

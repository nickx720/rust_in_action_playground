use std::{error::Error, fs::read, path::Path};

use rusqlite::{Connection, params};
use thiserror::Error;

use crate::md5::md5;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("rustsqlite error")]
    DBError(#[from] rusqlite::Error),
    #[error("file reading error")]
    FileIO(#[from] std::io::Error),
    #[error("wrapped main error")]
    Other(#[from] Box<dyn Error>),
}

pub fn setupconnpool() -> Result<Connection, BuildError> {
    let path = "assets/data.sqlite3";
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn dbsetup() -> Result<(), BuildError> {
    let path = "assets/data.sqlite3";

    if !Path::new(path).exists() {
        println!("Creating SQLite3 DB at {}", path);
        let mut conn = setupconnpool()?;
        conn.execute_batch(
            "CREATE TABLE cracked (
id INTEGER PRIMARY KEY,
original TEXT NOT NULL,
md5_hash TEXT NOT NULL
);
CREATE INDEX idx_md5 on cracked(md5_hash);
",
        )?;
        let file = read("assets/realhuman_phill.txt")?;
        let content: Vec<String> = String::from_utf8_lossy(&file)
            .split("\n")
            .map(|item| item.to_owned())
            .collect();
        dbg!(content.len());
        let transaction = conn.transaction()?;
        let mut stmt =
            transaction.prepare("INSERT INTO cracked (original, md5_hash) VALUES(?1, ?2)")?;
        for (index, item) in content.iter().enumerate() {
            let md5_hash = md5(item.clone());
            stmt.execute(params![item, md5_hash]).unwrap();
            println!("Committed item {}", index);
        }
        drop(stmt);
        transaction.commit()?; // so all the commits can be confirmed
        println!("DB has been initialized");
    } else {
        println!("SQLite DB already exists");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Content {
    pub original: String,
    pub md5_hash: String,
}

pub fn insert(content: Content) -> Result<(), BuildError> {
    let conn = setupconnpool()?;
    conn.execute(
        "INSERT INTO cracked (original, md5_hash) VALUES(?1,?2)",
        params![content.original, content.md5_hash],
    )?;
    Ok(())
}

pub fn get_query(hash: &str) -> Result<Content, BuildError> {
    let conn = setupconnpool()?;
    conn.query_row(
        "SELECT original,md5_hash FROM cracked where md5_hash = ?1",
        [hash],
        |row| {
            Ok(Content {
                original: row.get(0)?,
                md5_hash: row.get(1)?,
            })
        },
    )
    .map_err(BuildError::DBError)
}

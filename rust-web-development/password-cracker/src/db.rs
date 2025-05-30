use std::{fs::read, path::Path};

use rusqlite::{Connection, params};
use thiserror::Error;

use crate::md5::md5;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("rustsqlite error")]
    DBError(#[from] rusqlite::Error),
    #[error("file reading error")]
    FileIO(#[from] std::io::Error),
}

pub fn dbsetup() -> Result<(), BuildError> {
    let path = "assets/data.sqlite3";

    if !Path::new(path).exists() {
        println!("Creating SQLite3 DB at {}", path);
        let mut conn = Connection::open(path)?;

        conn.execute_batch(
            "CREATE TABLE cracked (
id INTEGER PRIMARY KEY,
original TEXT NOT NULL,
md5_hash BLOB NOT NULL
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
        dbg!("I am here");
        for (index, item) in content.iter().enumerate() {
            let hash = md5(item.clone());
            let md5_hash = hash.as_bytes();
            stmt.execute(params![item, md5_hash]).unwrap();
            println!("Committed item {}", index);
        }
        drop(stmt);
        transaction.commit()?; // so all the commits can be confirmed
        println!("DB has been initialized");
    } else {
        println!("SQLite DB already exists");
    }
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

pub fn get_query(hash: &str) -> Result<(), BuildError> {
    todo!()
}

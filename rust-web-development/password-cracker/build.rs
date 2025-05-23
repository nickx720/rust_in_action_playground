use std::path::Path;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let path = "assets/data.sqlite3";

    if !Path::new(path).exists() {
        println!("cargo:warning=Creating SQLite3 DB at {}", path);
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "CREATE TABLE cracked (
id INTEGER PRIMARY KEY,
original TEXT NOT NULL,
md5_hash BLOB NOT NULL
);
CREATE UNIQUE INDEX idx_md5 on cracked(md5_hash);
",
        )?;
        println!("cargo:warning=DB has been initialized");
    } else {
        println!("cargo:warning=SQLite DB already exists");
    }
    //    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

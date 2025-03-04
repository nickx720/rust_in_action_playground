pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
use actix_web::{Error, error, web};
use rusqlite::params;

use crate::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPrivacyStore {
    id: u32,
    original: String,
    token: String,
}

pub async fn initialize_db(pool: &Pool) -> Result<(), Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS value (
            id INTEGER PRIMARY KEY,
            original TEXT NOT NULL,
            token INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();
    Ok(())
}
// TODO handle error
pub async fn insert_token(pool: &Pool, values: DataPrivacyStore) -> Result<usize, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    let stmt = conn.execute(
        "
INSERT into vault (id,original,token) VALUES (?1,?2,?3)
",
        params![values.id, values.original, values.token],
    )?;
    Ok(stmt)
}

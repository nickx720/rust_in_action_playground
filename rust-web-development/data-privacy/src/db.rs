pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
use actix_web::{
    Error,
    error::{self, BlockingError, ErrorInternalServerError},
    web,
};
use rusqlite::params;
use thiserror::Error;

use crate::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPrivacyStore {
    id: u32,
    token: String,
}
impl DataPrivacyStore {
    pub fn new(id: u32, token: String) -> Self {
        Self { id, token }
    }
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("DB Error")]
    RusqLite(#[from] rusqlite::Error),
    #[error("R2D2 Error")]
    R2D2(#[from] r2d2::Error),
    #[error("BlockingError")]
    Blocking(#[from] BlockingError),
    #[error("Generic Actix Error")]
    GenericActixBlocking(#[from] Error),
}

pub async fn initialize_db(pool: &Pool) -> Result<(), DBError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(DBError::R2D2)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vault (
            id INTEGER PRIMARY KEY,
            token TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    Ok(())
}
pub async fn insert_token(pool: &Pool, values: DataPrivacyStore) -> Result<usize, DBError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(ErrorInternalServerError)?;
    let stmt = conn.execute(
        "
INSERT into vault (id,token) VALUES (?1,?2)
",
        params![values.id, values.token],
    )?;
    dbg!("I am here");
    Ok(stmt)
}
pub async fn get_token(pool: &Pool, id: u32) -> Result<DataPrivacyStore, DBError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(ErrorInternalServerError)?;
    let mut stmt = conn.prepare(
        "
SELECT id, token FROM vault where id = ?
",
    )?;
    stmt.query_row([id], |row| {
        let id = row.get(0)?;
        let token = row.get(1)?;
        Ok(DataPrivacyStore::new(id, token))
    })
    .map_err(DBError::RusqLite)
}

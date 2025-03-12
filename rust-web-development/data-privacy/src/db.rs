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
    original: String,
    token: String,
    nonce: Vec<u8>,
}
impl DataPrivacyStore {
    pub fn new(id: u32, original: String, token: String, nonce: Vec<u8>) -> Self {
        Self {
            id,
            original,
            token,
            nonce,
        }
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
            original TEXT NOT NULL,
            token TEXT NOT NULL,
            nonce BLOB NOT NULL
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
INSERT into vault (id,original,token,nonce) VALUES (?1,?2,?3,?4)
",
        params![values.id, values.original, values.token, values.nonce],
    )?;
    dbg!("I am here");
    Ok(stmt)
}

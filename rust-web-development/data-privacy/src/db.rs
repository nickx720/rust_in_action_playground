pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
use actix_web::{
    Error,
    error::{self, BlockingError, ErrorInternalServerError},
    web,
};
use rusqlite::params;
use serde_json::Value;
use thiserror::Error;

use crate::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPrivacyStore {
    id: u32,
    data: String,
}
impl DataPrivacyStore {
    pub fn new(id: u32, data: String) -> Self {
        Self { id, data }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPrivacyStoreResponse {
    id: u32,
    data: Value,
}
impl DataPrivacyStoreResponse {
    pub fn get_data(&self) -> Option<&serde_json::Map<String, Value>> {
        self.data.as_object()
    }
}
pub async fn initialize_db(pool: &Pool) -> Result<(), DBError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(DBError::R2D2)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vault (
            id INTEGER PRIMARY KEY,
            data TEXT NOT NULL
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
INSERT INTO vault (id, data) VALUES (?1,?2)
",
        params![values.id, values.data],
    )?;
    Ok(stmt)
}
pub async fn get_token(pool: &Pool, id: u32) -> Result<DataPrivacyStoreResponse, DBError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(ErrorInternalServerError)?;
    let mut stmt = conn.prepare(
        "
SELECT id, data FROM vault where id = ?
",
    )?;
    stmt.query_row([id], |row| {
        let id = row.get(0)?;
        let token: String = row.get(1)?;
        let data = serde_json::from_str(&token).unwrap();
        let data = DataPrivacyStoreResponse { id, data };
        Ok(data)
    })
    .map_err(DBError::RusqLite)
}

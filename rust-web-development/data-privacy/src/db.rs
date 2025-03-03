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

type DataPrivacyStoreError = Result<Vec<DataPrivacyStore>, rusqlite::Error>;

pub enum Queries {
    InsertTokens,
}

pub async fn execute(
    pool: &Pool,
    query: Queries,
    values: DataPrivacyStore,
) -> Result<Vec<DataPrivacyStore>, Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;
    web::block(move || match query {
        Queries::InsertTokens => insert_token(conn, values),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
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

fn insert_token(conn: Connection, values: DataPrivacyStore) -> DataPrivacyStoreError {
    let stmt = conn.execute(
        "
INSERT into vault (id,original,token) VALUES (?1,?2,?3)
",
        params![values.id, values.original, values.token],
    )?;
    todo!()
}

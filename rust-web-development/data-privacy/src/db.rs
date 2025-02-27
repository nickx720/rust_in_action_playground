pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
use actix_web::Error;

use crate::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPrivacyStore {
    id: u32,
    original: String,
    token: String,
}

pub enum Queries {}

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<DataPrivacyStore>, Error> {
    todo!()
}

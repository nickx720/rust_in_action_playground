use std::error::Error;

use crate::db::{dbsetup, get_query};

//2bdb742fc3d075ec6b73ea414f27819a
pub fn wordlist_reader(hash: &str) -> Result<String, Box<dyn Error>> {
    let _ = dbsetup();
    let response = get_query(hash.trim())?;
    Ok(response.original)
}

pub mod models;
pub mod schema;
pub mod database_utils;
pub mod utils;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use anyhow::{Result, Context};

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .with_context(|| "DATABASE_URL must be set")?;
    SqliteConnection::establish(&database_url)
        .with_context(|| format!("Error connecting to {}", database_url))
}
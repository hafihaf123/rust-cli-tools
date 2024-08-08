pub mod models;
pub mod database_utils;
pub mod utils;
pub mod schema;
pub mod endings;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use anyhow::{Context, Result};

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .with_context(|| "DATABASE_URL must be set")?;
    SqliteConnection::establish(&database_url)
        .with_context(|| format!("Error connecting to {}", database_url))
}
// src/config.rs
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
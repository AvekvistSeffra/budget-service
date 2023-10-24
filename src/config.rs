use serde::{Deserialize, Serialize};
use urlencoding::encode;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub ip: [u8; 4],
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(rename = "pool-size")]
    pub pool_size: u32,
    pub timeout: u64,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            encode(&self.user), encode(&self.password), encode(&self.host), self.port, encode(&self.database)
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub address: Address,
    pub database: Database,
}

impl Config {
    pub fn load(path: &str) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

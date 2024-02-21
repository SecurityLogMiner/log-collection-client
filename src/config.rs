use std::io::Read;
use std::fs::File;
use toml;
use serde_derive::{Deserialize,Serialize};

#[derive(Debug, Deserialize)]
pub struct Sources {
    pub logs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DynamoDBConfig {
    pub table: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dynamodb: DynamoDBConfig,
    pub sources: Sources,
}

pub fn
read_config() -> Option<Config> {
    let mut file = File::open("config.toml").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let config: Config = toml::from_str(&data).unwrap();
    Some(config)
}


use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{fs::File, io::Read};

pub fn timestamp() -> i64 {
    let now: DateTime<Utc> = Utc::now();
    let timestamp_millis = now.timestamp_millis();
    timestamp_millis
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client_app_id: String,
    pub authorization: String,
    pub cookies: String,
    pub user_agent: String,
    pub faction: Vec<String>,
    pub rarity: Vec<String>,
    pub foil: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut file = File::open("config.toml").expect("unable to load config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("unable to read file");
        let config = toml::from_str(&contents).expect("unable to parse config file");
        config
    }
}

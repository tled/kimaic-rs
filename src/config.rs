use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XAuthUser {
    pub user: String,
    pub token: String
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub auth: XAuthUser,
}

pub fn load_cfg() -> Result<Config, String> {
    let f = File::open("config.yaml").expect("Could not open config file.");
    let cfg: Config = serde_yaml::from_reader(f).expect("Error loading config file.");
    return Ok(cfg);
}

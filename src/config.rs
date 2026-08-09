use std::fs;
use serde::Deserialize;

use crate::Server;



pub fn get_servers() -> Vec<Server>{

    println!("reading server config");

    let content = fs::read_to_string("config.toml").expect("failed to read config.toml");
    let config: Config = toml::from_str(&content).expect("failed to read config content");

    return config.servers;
}



#[derive(Deserialize)]
struct Config {
servers: Vec<Server>,
}

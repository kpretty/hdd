extern crate serde_yaml;
extern crate serde;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    version: String,
    services: HashMap<String, InnerServer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InnerServer {
    env_file: Vec<String>,
    image: String,
    hostname: String,
    container_name: String,
    volumes: Vec<String>,
    ports: Vec<String>,
    command: Vec<String>,
}
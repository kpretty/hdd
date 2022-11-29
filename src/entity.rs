extern crate serde_yaml;
extern crate serde;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub version: String,
    pub services: HashMap<String, InnerServer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerServer {
    pub env_file: Vec<String>,
    pub image: String,
    pub hostname: String,
    pub container_name: String,
    pub volumes: Vec<String>,
    pub ports: Vec<String>,
    pub command: Vec<String>,
}
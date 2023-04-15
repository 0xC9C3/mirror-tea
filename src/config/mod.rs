use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub watcher_interval_ms: u64,
    pub targets: HashMap<String, Target>,
    pub sources: HashMap<String, Source>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "type")]
    pub type_: TargetType,
    pub url: String,
    pub auth: Auth,
    pub issues: bool,
    pub labels: bool,
    pub milestones: bool,
    pub mirror_interval: String,
    pub private: bool,
    pub pull_requests: bool,
    pub releases: bool,
    pub wiki: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub type_: SourceType,
    pub token: String,
    pub targets: Vec<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Github,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Gitea,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watcher_interval_ms: 30000,
            targets: Default::default(),
            sources: Default::default(),
        }
    }
}

pub fn generate_default_config(path: &str) {
    if Path::new(path).exists() {
        panic!("Config file already exists");
    }

    let config = Config::default();
    let config_str = serde_yaml::to_string(&config)
        .expect("Failed to serialize config");
    fs::write(path, config_str).expect("Failed to write config file");
}

pub fn load_config(path: &str) -> Config {
    let config_str = fs::read_to_string(path)
        .expect("Failed to read config file");
    serde_yaml::from_str(&config_str)
        .expect("Failed to parse config")
}


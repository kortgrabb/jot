use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    // TODO: Add fields for configuration
}

impl Default for Config {
    fn default() -> Self {
        Config { }
    }
}

impl Config {
    pub fn new() -> Self {
        Config { }
    }
}
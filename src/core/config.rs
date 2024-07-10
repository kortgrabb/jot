use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub save_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config::new(String::from("./save.json"))
    }
}

impl Config {
    pub fn new(save_path: String) -> Config {
        Config { save_path }
    }
}

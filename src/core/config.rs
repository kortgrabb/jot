use serde::{Deserialize, Serialize};

const DEFAULT_SAVE_PATH: &str = "./save.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    // set default for serde to ./save.json
    #[serde(default = "default_save_path")]
    pub save_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            save_path: default_save_path(),
        }
    }
}

impl Config {
    pub fn new(save_path: String) -> Config {
        Config { save_path }
    }
}

fn default_save_path() -> String {
    String::from(DEFAULT_SAVE_PATH)
}

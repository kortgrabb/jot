use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

const DEFAULT_SAVE_PATH: &str = "./save.json";

#[derive(Serialize, Deserialize, Clone)]
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

fn get_save_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::data_local_dir().ok_or("No local data directory")?;
    path.push("jot");
    // Create the directory if it doesn't exist
    fs::create_dir_all(&path)?;
    
    Ok(path)
}

fn default_save_path() -> String {
    match get_save_directory() {
        Ok(mut path) => {
            path.push("journal.json");
            path.to_str().unwrap().to_string()
        }
        Err(_) => DEFAULT_SAVE_PATH.to_string(),
    }
}

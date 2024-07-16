use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::journal::{self, Journal};

pub struct SaveManager {
    pub save_path: String,
}

impl Default for SaveManager {
    fn default() -> Self {
        SaveManager {
            save_path: default_save_path(),
        }
    }
}

impl SaveManager {
    
    pub fn new() -> Self {
        SaveManager { 
            save_path: default_save_path(),
        }
    }

    pub fn with_path(path: String) -> Self {
        SaveManager { save_path: path }
    }

    pub fn save_json<T>(&self, data: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let json = serde_json::to_string_pretty(&data)?;
        let mut file = File::create(&self.save_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    // TODO: Move loading to here
}

fn get_save_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::data_local_dir().ok_or("No local data directory")?;
    path.push("jot");
    // Create the directory if it doesn't exist
    fs::create_dir_all(&path)?;
    Ok(path)
}


fn default_save_path() -> String {
    // Check if we are in debug mode
    #[cfg(debug_assertions)]
    let path = PathBuf::from("./journal.json");

    //Check if we are in release mode
    #[cfg(not(debug_assertions))]
    let path = {
        match get_save_directory() {
            Ok(mut path) => {
                path.push("journal.json");
                path
            }
            Err(_) => PathBuf::from("journal.json"),
        }
    };

    path.to_str().unwrap().to_string()
}
use std::fs::File;
use std::io::Write;
use serde::Serialize;
use crate::core::config::Config;

pub struct SaveManager {
    pub cfg: Config
}

impl Default for SaveManager {
    fn default() -> Self {
        SaveManager {
            cfg: Config::default()
        }
    }
}

impl SaveManager {
    pub fn new(cfg: Config) -> SaveManager {
        SaveManager {
            cfg
        }
    }

    pub fn with_config(cfg: Config) -> SaveManager {
        SaveManager {
            cfg
        }
    }

    pub fn save_json<T>(&self, data: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let json = serde_json::to_string_pretty(&data)?;
        let mut file = File::create(&self.cfg.save_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
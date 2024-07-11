use std::{
    fs::File,
    io::{Error, Write},
};

use serde::{Deserialize, Serialize};

use crate::utils::helpers;

use super::{config::Config, entry::Entry};

#[derive(Serialize, Deserialize)]
pub struct Journal {
    entries: Vec<Entry>,
    cfg: Config,
}

impl Default for Journal {
    fn default() -> Self {
        Self::new()
    }
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            entries: vec![],
            cfg: Config::default(),
        }
    }

    pub fn with_config(cfg: Config) -> Self {
        Journal {
            entries: vec![],
            cfg,
        }
    }

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, id: usize) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub fn add_entry(&mut self, mut entry: Entry) -> Result<(), Error> {
        entry.id = self.entries.len();

        self.entries.push(entry);
        self.save_json()?;
        Ok(())
    }

    pub fn remove_entry(&mut self, id: usize) -> Result<(), Error> {
        if id >= self.entries.len() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Entry with id {} not found", id),
            ));
        }

        self.entries.remove(id);
        self.save_json()?;

        Ok(())
    }

    pub fn list_entries(&mut self, start_date: &str, end_date: &str) {
        let start = helpers::parse_date(start_date);
        let end = helpers::parse_date(end_date);

        for entry in &self.entries {
            if entry.date >= start && entry.date <= end {
                println!("{} {}", entry.date, entry.title);
                println!("{}\n", entry.content);
            }
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn as_raw(&self, input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    pub fn save_json(&self) -> Result<(), std::io::Error> {
        let json = self
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = File::create(&self.cfg.save_path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn load_entries(&mut self) -> Result<(), std::io::Error> {
        let raw = std::fs::read_to_string(&self.cfg.save_path);
        match raw {
            Ok(r) => {
                if r.is_empty() {
                    self.save_json()?;
                    return Ok(());
                }

                let loaded = self.as_raw(r.as_str())?;
                self.entries = loaded.entries;
            }

            Err(_) => {
                self.save_json()?;
            }
        }

        Ok(())
    }
}
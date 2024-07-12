use std::collections::HashMap;
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
    unique_tags: Option<HashMap<String, u32>>,
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
            unique_tags: None,
        }
    }

    pub fn with_config(cfg: Config) -> Self {
        Journal {
            entries: vec![],
            cfg,
            unique_tags: None,
        }
    }

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }

    pub fn load_entries(&mut self) -> Result<(), std::io::Error> {
        let raw = std::fs::read_to_string(&self.cfg.save_path)?;
        if raw.is_empty() {
            // Optionally initialize unique_tags here if desired
            self.save_json()?;
            return Ok(());
        }

        let loaded: Journal = serde_json::from_str(&raw)?;
        self.entries = loaded.entries;
        self.unique_tags = loaded.unique_tags;

        Ok(())
    }

    pub fn tags_field_present(&self) -> bool {
        self.unique_tags.is_some()
    }

    pub fn update_unique_tags(&mut self) {
        let mut tag_map: HashMap<String, u32> = HashMap::new();
        for entry in &self.entries {
            for tag in &entry.tags {
                *tag_map.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        self.unique_tags = Some(tag_map);
    }

    pub fn list_unique_tags(&mut self) {
        if let Some(tags) = &self.unique_tags {
            for (tag, count) in tags {
                println!("@{}: {}", tag, count);
            }
        }

        self.save_json().unwrap();
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

    // fn as_raw(&self, input: &str) -> Result<Self, serde_json::Error> {
    //     serde_json::from_str(input)
    // }

    pub fn save_json(&mut self) -> Result<(), std::io::Error> {
        let json = self.to_json()?;
        let mut file = File::create(&self.cfg.save_path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

use std::collections::HashMap;
use std::io::Error;

use crate::core::savemanager::SaveManager;
use crate::utils::helpers;
use serde::{Deserialize, Serialize};

use super::{config::Config, entry::Entry};

#[derive(Serialize, Deserialize)]
pub struct Journal {
    entries: Vec<Entry>,
    cfg: Config,
    unique_tags: Option<HashMap<String, u32>>,

    #[serde(skip)]
    pub save_manager: SaveManager,
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
            save_manager: SaveManager::default(),
        }
    }

    pub fn with_config(cfg: Config) -> Self {
        Journal {
            entries: vec![],
            cfg: cfg.clone(),
            unique_tags: None,
            save_manager: SaveManager::new(cfg),
        }
    }

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }

    pub fn load_entries(&mut self) -> Result<(), std::io::Error> {
        let raw = match std::fs::read_to_string(&self.cfg.save_path) {
            Ok(raw) => raw,
            Err(_) => {
                self.save_manager
                    .save_json(&self)
                    .expect("Failed to save journal");
                return Ok(());
            }
        };

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
        self.update_unique_tags();
        if let Some(tags) = &self.unique_tags {
            for (tag, count) in tags {
                println!("@{}: {}", tag, count);
            }
        }
    }

    pub fn get_entry(&self, id: usize) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub fn add_entry(&mut self, mut entry: Entry) -> Result<(), Error> {
        entry.id = self.entries.len();

        self.entries.push(entry);
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
}

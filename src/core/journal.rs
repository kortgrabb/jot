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

    pub fn entry_size(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, id: usize) -> Option<&Entry> {
        self.entries.get(id)
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Error> {
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
                println!("{}", entry.content);
            }
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn load_from_json(&self, input: &str) -> Result<Self, serde_json::Error> {
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

                let loaded = self.load_from_json(r.as_str())?;
                self.entries = loaded.entries;
            }

            Err(_) => {
                self.save_json()?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_journal_is_empty() {
        let journal = Journal::new();
        assert_eq!(journal.entries.len(), 0);
    }

    #[test]
    fn test_add_entry() {
        let mut journal = Journal::new();
        let entry = Entry::new(
            "Test entry".to_string(),
            vec!["test".to_string()],
            journal.entries.len(),
        );
        let _ = journal.add_entry(entry);
        assert_eq!(journal.entries.len(), 1);
    }

    #[test]
    fn test_multiple_entries() {
        let mut journal = Journal::new();
        let _ = journal.add_entry(Entry::new(
            "Entry 1".to_string(),
            vec![],
            journal.entries.len(),
        ));
        let _ = journal.add_entry(Entry::new(
            "Entry 2".to_string(),
            vec![],
            journal.entries.len(),
        ));
        assert_eq!(journal.entries.len(), 2);
    }
}

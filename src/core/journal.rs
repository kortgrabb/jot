use std::io::Error;

use super::entry::Entry;

pub struct Journal {
    entries: Vec<Entry>
}

impl Journal {
    pub fn new() -> Self {
        Journal { entries: vec![] }
    }
    
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Error> {
        self.entries.push(entry);

        Ok(())
    }

    pub fn list_entries(&mut self) -> Result<(), Error> {

        println!("entries:");
        for (id, entry) in self.entries.iter().enumerate() {
            println!("entry {}: {}", id, entry.content);
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
        let entry = Entry::new("Test entry".to_string(), vec!["test".to_string()]);
        let _ = journal.add_entry(entry);
        assert_eq!(journal.entries.len(), 1);
    }

    #[test]
    fn test_multiple_entries() {
        let mut journal = Journal::new();
        let _ = journal.add_entry(Entry::new("Entry 1".to_string(), vec![]));
        let _ = journal.add_entry(Entry::new("Entry 2".to_string(), vec![]));
        assert_eq!(journal.entries.len(), 2);
    }
}
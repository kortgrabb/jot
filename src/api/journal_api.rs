use std::io::Error;

use crate::core::{entry::Entry, journal::Journal, logger::Logger};

pub struct JournalAPI {
    journal: Journal
}

impl JournalAPI {
    pub fn new() -> Self {
        JournalAPI { journal: Journal::new() }
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Error> {
        self.journal.add_entry(entry)
    }
}
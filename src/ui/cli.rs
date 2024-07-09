use std::collections::HashMap;

use crate::{api::journal_api::{self, JournalAPI}, core::{entry::Entry, journal::Journal}};

pub struct Cli {
    jrnl: Journal
}

impl Cli {
    pub fn new() -> Cli {
        Cli { jrnl: Journal::new() }
    }

    pub fn run(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("Error: No command provided");
            println!("Usage: jot <command> [options] [arguments]");
            println!("Commands: add, remove, list");
            return;
        }

        match args[0].as_str() {
            "write" => self.add_entry(&args[1..]),
            "remove" => self.remove_entry(&args[1..]),
            "list" => self.list_entries(&args[1..]),
            _ => println!("Error: Unknown command: {}", args[0]),
        }
    }

    fn add_entry(&mut self, args: &[String]) {
        let (options, content) = self.parse_options(args);
        
        let date = options.get("date").map(String::as_str).unwrap_or("today");
        let mood = options.get("mood").map(String::as_str).unwrap_or("neutral");

        println!(
            "Adding entry: content={:?}, date={}, mood={}",
            content, date, mood
        );

        let entry = Entry::new(content.join(" ").to_string(), vec![]);
        self.jrnl.add_entry(entry);
    }

    fn remove_entry(&mut self, args: &[String]) {
        println!("Removing entry: {:?}", args);
        // Implementation for removing an entry
    }

    fn list_entries(&mut self, args: &[String]) {
        println!("Listing entries with args: {:?}", args);
        let (options, _) = self.parse_options(args);

        let start_date = options.get("start").map(String::as_str).unwrap_or("today");

        println!("listing from {}", start_date);
    }

    fn parse_options<'a>(&self, args: &'a [String]) -> (HashMap<String, String>, Vec<&'a str>) {
        let mut options = HashMap::new();
        let mut content = Vec::new();
        let mut i = 0;

        while i < args.len() {
            if args[i].starts_with("--") {
                let key = &args[i][2..];
                i += 1;
                if i < args.len() {
                    options.insert(key.to_string(), args[i].to_string());
                }
            } else {
                content.push(args[i].as_str());
            }
            i += 1;
        }

        (options, content)
    }
}
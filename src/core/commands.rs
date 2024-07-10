use crate::{
    core::{entry::Entry, journal::Journal, parser::parse_options},
    utils::helpers,
};
use std::collections::HashMap;

pub trait Command {
    fn execute(&self, jrnl: &mut Journal, args: &[String]);
}

pub struct AddEntry;
pub struct RemoveEntry;
pub struct ListEntries;

impl Command for AddEntry {
    fn execute(&self, jrnl: &mut Journal, args: &[String]) {
        let (options, content) = parse_options(args);
        let content = content.join(" ");
        let mut entry = Entry::new(content, Vec::new(), jrnl.entry_size());

        let defaults: HashMap<&str, Option<&str>> = [
            ("date", Some("today")),
            ("mood", None),
            ("weather", None),
            ("location", None),
        ]
        .iter()
        .cloned()
        .collect();

        for (key, default_value) in defaults {
            let value = options.get(key).map(String::as_str).or(default_value);
            set_entry_property(&mut entry, key, value);
        }

        match jrnl.add_entry(entry) {
            Ok(_) => println!("Entry added successfully"),
            Err(e) => eprintln!("Error adding entry: {}", e),
        }
    }
}

impl Command for RemoveEntry {
    fn execute(&self, jrnl: &mut Journal, args: &[String]) {
        let _ = jrnl;
        let (options, _) = parse_options(args);
        println!("Removing entry with options: {:?}", options);
    }
}

impl Command for ListEntries {
    fn execute(&self, jrnl: &mut Journal, args: &[String]) {
        let (options, _) = parse_options(args);

        let start = options.get("from").map(String::as_str).unwrap_or("start");
        let end = options.get("to").map(String::as_str).unwrap_or("today");

        println!("listing entries between {} and {}", start, end);
        jrnl.list_entries(start, end);
    }
}

fn set_entry_property(entry: &mut Entry, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|&v| !v.is_empty()) {
        match key {
            "date" => entry.set_date(helpers::parse_date(value)),
            "mood" => entry.set_mood(value),
            "weather" => entry.set_weather(value),
            "location" => entry.set_location(value),
            _ => eprintln!("Unknown option: {}", key),
        }
    }
}

pub fn get_command(command: &str) -> Option<Box<dyn Command>> {
    // TODO: Reduce "parse_options" calls by moving it to "get_command"\
    match command {
        "add" => Some(Box::new(AddEntry)),
        "remove" => Some(Box::new(RemoveEntry)),
        "list" => Some(Box::new(ListEntries)),
        _ => {
            eprintln!("Unknown command: {}", command);
            None
        }
    }
}

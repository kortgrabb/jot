use crate::core::entry::EntryBuilder;
use crate::core::journal::Journal;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Write,
    Remove,
    List,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "write" => Ok(Command::Write),
            "remove" => Ok(Command::Remove),
            "list" => Ok(Command::List),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

impl Command {
    pub fn execute(&self, jrnl: &mut Journal, args: &[String]) {
        match self {
            Command::Write => self.add_entry(jrnl, args),
            Command::Remove => self.remove_entry(jrnl, args),
            Command::List => self.list_entries(jrnl, args),
        }
    }

    fn add_entry(&self, jrnl: &mut Journal, args: &[String]) {
        let (options, content) = parse_options(args);
        let content = content.join(" ");

        let entry = EntryBuilder::default()
            .content(content)
            .mood(options.get("mood").cloned())
            .weather(options.get("weather").cloned())
            .location(options.get("location").cloned())
            .build()
            .unwrap_or_else(|e| {
                eprintln!("Error building entry: {}", e);
                std::process::exit(1);
            });

        match jrnl.add_entry(entry) {
            Ok(_) => println!("Entry added successfully"),
            Err(e) => eprintln!("Error adding entry: {}", e),
        }
    }

    fn remove_entry(&self, jrnl: &mut Journal, args: &[String]) {
        let (options, _) = parse_options(args);

        if let Some(id_string) = options.get("id") {
            match id_string.parse::<usize>() {
                Ok(id) => match jrnl.remove_entry(id) {
                    Ok(_) => println!("Entry removed successfully"),
                    Err(e) => eprintln!("Error removing entry: {}", e),
                },
                Err(_) => eprintln!("Invalid entry ID"),
            }
        } else {
            eprintln!("Please provide an entry ID to remove");
        }
    }

    fn list_entries(&self, jrnl: &mut Journal, args: &[String]) {
        let (options, _) = parse_options(args);

        let start = options.get("from").map(String::as_str).unwrap_or("start");
        let end = options.get("to").map(String::as_str).unwrap_or("today");

        println!("Listing entries between {} and {}", start, end);
        jrnl.list_entries(start, end);
    }
}

pub fn get_command(command: &str) -> Option<Command> {
    command.parse().ok()
}

pub fn parse_options(args: &[String]) -> (HashMap<String, String>, Vec<&str>) {
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

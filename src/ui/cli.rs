use std::collections::HashMap;

pub struct Cli;

impl Cli {
    pub fn new() -> Cli {
        Cli
    }

    pub fn run(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("Error: No command provided");
            println!("Usage: jot <command> [options] [arguments]");
            return;
        }

        match args[0].as_str() {
            "add" => self.add_entry(&args[1..]),
            "remove" => self.remove_entry(&args[1..]),
            "list" => self.list_entries(&args[1..]),
            _ => println!("Error: Unknown command: {}", args[0]),
        }
    }

    fn add_entry(&mut self, args: &[String]) {
        let (options, content) = self.parse_options(args);
        
        let date = options.get("date").cloned().unwrap_or_else(|| "today".to_string());
        let mood = options.get("mood").cloned().unwrap_or_else(|| "neutral".to_string());

        println!(
            "Adding entry: content={:?}, date={}, mood={}",
            content, date, mood
        );

    }

    fn remove_entry(&mut self, args: &[String]) {
        println!("Removing entry: {:?}", args);
        // Implementation for removing an entry
    }

    fn list_entries(&mut self, args: &[String]) {
        println!("Listing entries with args: {:?}", args);
        // Implementation for listing entries
    }

    fn parse_options(&self, args: &[String]) -> (HashMap<String, String>, Vec<String>) {
        let mut options = HashMap::new();
        let mut content = Vec::new();
        let mut i = 0;

        while i < args.len() {
            if args[i].starts_with("--") {
                let key = args[i][2..].to_string();
                i += 1;
                if i < args.len() {
                    options.insert(key, args[i].to_string());
                }
            } else {
                content.push(args[i].clone());
            }
            i += 1;
        }

        (options, content)
    }
}
use crate::{
    core::commands::{get_command, Command},
    core::journal::Journal,
};

pub struct Cli {
    jrnl: Journal,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            jrnl: Journal::new(),
        }
    }

    pub fn run(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("Error: No command provided");
            println!("Usage: jot <command> [options] [arguments]");
            println!("Commands: write, remove, list");
            return;
        }

        self.jrnl.load_entries().unwrap();

        match get_command(&args[0]) {
            Some(command) => command.execute(&mut self.jrnl, &args[1..]),
            None => println!("Error: Unknown command: {}", args[0]),
        }
    }
}

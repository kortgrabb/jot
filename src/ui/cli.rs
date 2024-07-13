use crate::{core::commands::get_command, core::journal::Journal};

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

    // Parse the command and execute it
    pub fn run(&mut self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            println!("Error: No command provided");
            println!("Usage: jot <command> [args]");
            println!("Commands: write, remove, list");
            return Ok(());
        }

        self.jrnl.load_entries()?;
        let command = get_command(&args[0])?;
        command.execute(&mut self.jrnl, &args[1..]);

        Ok(())
    }
}

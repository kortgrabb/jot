use crate::core::logger::{Logger};

pub struct Cli {
    logger: Logger,
}

impl Cli {
    pub fn new() -> Cli {
        Cli { logger: Logger::new("cli.rs") }
    }

    pub fn run(&mut self) {
        self.logger.log("I am running! 🏃")
    }
}
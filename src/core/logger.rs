use std::fmt::{self, write};

pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
    DEBUG
}

pub struct Logger {
    source: String,
    log_file: Option<String> // ! IMPLEMENT LATER
}

impl Logger {
    pub fn new(source: &str) -> Logger {
        Logger { source: source.to_string(), log_file: None }
    }

    pub fn log(&mut self, message: &str, level: LogLevel) {
        let loc_time = chrono::Local::now();
        println!("[{}] {} -- {} at {}", level, message, self.source, loc_time);
    }
}

impl fmt::Display for LogLevel {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => { write!(f, "INFO") }
            LogLevel::WARNING => { write!(f, "WARNING") }
            LogLevel::ERROR => { write!(f, "ERROR") }
            LogLevel::DEBUG => { write!(f, "DEBUG") }
        }
   }
}
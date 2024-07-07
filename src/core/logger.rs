pub struct Logger {
    source: String
}

impl Logger {
    pub fn new(source: &str) -> Logger {
        Logger { source: source.to_string() }
    }

    pub fn log(&mut self, message: &str) {
        println!("{} --/ {}", message, self.source);
    }
}
use jot::ui::cli::Cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cli = Cli::default();

    println!("hello, {}", "world");

    if let Err(e) = cli.run(&args) {
        eprintln!("Error: {}", e);
    }
}

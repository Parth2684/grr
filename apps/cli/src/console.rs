use std::fmt::Display;

use console::style;




pub struct Console;

impl Console {
    pub fn success(message: impl Display) {
        println!("{} {}", style("✅").bold(), message)
    }
    pub fn error(message: impl Display) {
        println!("{} {}", style("❌").red().bold(), style(message).red())
    }
    pub fn warn(message: impl Display) {
        println!("{} {}", style("!").yellow().bold(), message)
    }
    pub fn info(message: impl Display) {
        println!("{} {}", style("➡️").cyan().bold(), message)
    }
}
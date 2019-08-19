use ansi_term::Colour::{Green, Red, Yellow};

use crate::error::SageError;

// Prints message that the job's done.
pub fn print_command_done() {
    println!("[{}] Done.", Green.paint("OK"))
}

// Prints regular message in terminal.
pub fn print_info(message: &str) {
    println!("[{}] {}", Green.paint("INFO"), message)
}

// Prints warning message in terminal.
pub fn print_warning(message: &str) {
    println!("[{}] {}", Yellow.paint("WARNING"), message)
}

// Prints error message in terminal.
pub fn print_error(err: SageError) {
    println!("[{}] {}", Red.paint("ERROR"), err)
}

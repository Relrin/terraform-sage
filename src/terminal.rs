use ansi_term::Colour::{Green, Red, Yellow};

use crate::error::SageError;

pub fn print_command_done() {
    println!("[{}] Done.", Green.paint("OK"))
}

pub fn print_info(message: &str) {
    println!("[{}] {}", Green.paint("INFO"), message)
}

pub fn print_warning(message: &str) {
    println!("[{}] {}", Yellow.paint("WARNING"), message)
}

pub fn print_error(err: SageError) {
    println!("[{}] {}", Red.paint("ERROR"), err)
}

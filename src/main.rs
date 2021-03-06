mod cli;
mod client;
mod error;
mod template;
mod terminal;
mod terraform;
mod utils;

use structopt::StructOpt;

use crate::cli::Command;
use crate::client::SageClient;

fn main() {
    let command = Command::from_args();
    let client = SageClient::new();
    client.run(&command);
}

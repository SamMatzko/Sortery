mod messages;
mod tools;

use messages::error_messages;
use std::{env};
use tools::command_line::CommandLineParser;

fn main() {

    // The command-line arguments
    let args: Vec<String> = env::args().collect();

    // Parse the arguments
    let parser = CommandLineParser { args };
    let result = parser.parse();
}
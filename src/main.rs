mod messages;
mod tools;

use messages::info_messages;
use std::env;
use tools::command_line::CommandLineParser;

fn main() {

    // The command-line arguments
    let args: Vec<String> = env::args().collect();

    // Parse the arguments
    let parser = CommandLineParser { args };
    let result = parser.parse();

    // Show the help message if the user specified to
    if result.help {
        info_messages::help();
        return;
    }
    
    // Print overall error information
    if result.errors >= 1 {
        println!("Failed to execute command: {} errors found.", result.errors);
        return;
    }

    // Run according to the command-line options
    if result.extract {
        tools::extract(result.source, result.target);
    } else if result.sort {
        tools::sort::by_date(result.source, result.target);
    }
}
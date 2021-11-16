mod messages;
mod tools;

use messages::error_messages;
use std::{env};

fn help(version_number: String) {
    // Display the help messages and exit

    println!("Coral {}\n", version_number);
    println!("Coral is a free, open-source file and directory sorter written");
    println!("in Rust.\n");
    println!("usage: [SOURCE DIR] [TARGET DIR] [-h, --help]\n");
    println!("Command-line options:\n");
    print!("-h, --help\t");
    println!("Show this message and quit.");
}

fn main() {

    // Program constants
    let options = [
        String::from("-h"),
        String::from("--help")
    ];
    let version_number = "0.0.1";

    // The command-line arguments
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    // println!("{:?}", args);

    // If there are no arguments, show the help message
    if args.len() == 0 {
        help((&version_number).to_string());
        return;
    }
    
    // Check all arguments for validity
    for option in args {

        // Fist check the options (those starting with "-" or "--")
        if option.starts_with("-") || option.starts_with("--") {
            if !options.contains(option) {
                error_messages::UnknownOptionError { option: option.to_string() }.show();
            }
        }
    }
    if args.contains(&options[0]) || args.contains(&options[1]) {
        // -h --help: Show the help message
        help((&version_number).to_string());
    }
}

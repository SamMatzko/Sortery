use colored::Colorize;
use std::env;

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

fn unknown_option(arg: &String) {
    // Print the "unknown option" message and exit

    println!("{} unknown option: {}. Try coral --help for more info.", format!("Error:").red(), arg);
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
        if !options.contains(option) {
            unknown_option(option);
        }
    }
    
    if args.contains(&options[0]) || args.contains(&options[1]) {
        help((&version_number).to_string());
    }
}

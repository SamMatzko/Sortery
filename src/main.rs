use colored::Colorize;
use std::env;
use std::path::Path;

fn check_exists(path: &Path) -> bool {
    // Check if the given path actually exists, and print an error if it doesn't

    if !path.exists() {
        println!(
            "{0} \"{1}\": no such file or directory. Try coral --help for more info.",
            format!("Error:").red(),
            path.display()
        );
    }
    path.exists()
}

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
    let mut paths_exist = true;
    for option in args {

        // Fist check the options (those starting with "-" or "--")
        if option.starts_with("-") || option.starts_with("--") {
            if !options.contains(option) {
                unknown_option(option);
            }

        // Then check the path arguments (those starting with anything else)
        } else {
            let path = Path::new(option);
            if !check_exists(&path) {
                paths_exist = false;
            }
        }
    }

    // End the execution if any of the paths were invalid
    if !paths_exist {
        return;
    }
    
    if args.contains(&options[0]) || args.contains(&options[1]) {
        help((&version_number).to_string());
    }
}

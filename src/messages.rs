pub mod error_messages {
    // Error message templates for common errors
    
    use colored::Colorize;
    use std::path::Path;

    // When the user inputs a file or directory that doesn't exist
    pub struct PathDoesNotExistError {
        pub path: Path,
    }
    impl PathDoesNotExistError {
        pub fn show(&self) {
            // Print the error message

            println!(
                "{} no such file or directory \"{}\". Try coral --help for more info.",
                format!("Error:").red(),
                self.path.display()
            )
        }
    }

    // When the user inputs an unknown command-line option
    pub struct UnknownOptionError {
        pub option: String,
    }
    impl UnknownOptionError {
        pub fn show(&self) {
            // Print the error message

            println!(
                "{} unknown option \"{}\". Try coral --help for more info.",
                format!("Error:").red(),
                self.option
            )
        }
    }
}

pub mod info_messages {
    // Messages for information and help
    pub fn help() {
        // Display the help messages and exit

        let version_number = String::from("0.0.1");
    
        println!("Coral {}\n", version_number);
        println!("Coral is a free, open-source file and directory sorter written");
        println!("in Rust.\n");
        println!("usage: [SOURCE DIR] [TARGET DIR] [-h, --help]\n");
        println!("Command-line options:\n");
        print!("-h, --help\t");
        println!("Show this message and quit.");
    }    
}
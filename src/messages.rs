pub mod error_messages {
    // Error message templates for common errors
    
    use colored::Colorize;
    use std::path::Path;

    // When the user does not input enough arguments
    pub struct NotEnoughArgsError {
        pub len: usize,
    }
    impl NotEnoughArgsError {
        pub fn show(&self) {
            // Print the error message

            println!(
                "{} expected at least 3 arguments, got {}. Try coral --help for more info.",
                format!("Error:").red(),
                self.len.to_string()
            );
        }
    }

    // When the user inputs a file or directory that doesn't exist
    pub struct PathDoesNotExistError <'a> {
        pub path: &'a Path,
    }
    impl <'a> PathDoesNotExistError <'a> {
        pub fn show(&self) {
            // Print the error message

            println!(
                "{} no such file or directory \"{}\". Try coral --help for more info.",
                format!("Error:").red(),
                self.path.display()
            );
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
        println!("\tusage: [SOURCE DIR] [TARGET DIR] [-h, --help] <args>\n");
        println!("Command-line options:\n");
        
        // The command-line options
        
        // Extract
        print!("\t-e, --extract\t");
        println!("Extract all the contents of SOURCE to TARGET.");

        // Help
        print!("\t-h, --help\t");
        println!("Show this message and quit.");

    }    
}
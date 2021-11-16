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
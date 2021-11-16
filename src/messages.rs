pub mod error_messages {
    // Error message templates for common errors
    
    use colored::Colorize;
    use std::path::Path;

    pub struct PathDoesNotExistError {
        // When the user inputs a file or directory that doesn't exist
        path: Path,
    }
    impl PathDoesNotExistError {
        fn show(&self) {
            // Print the error message

            println!(
                "{} \"{}\": no such file or directory. Try colal --help for more info.",
                format!("Error:").red(),
                self.path.display()
            )
        }
    }
}
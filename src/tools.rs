use std::path::Path;

// Commmand-line tools
pub mod command_line {

    use super::super::messages::{error_messages, info_messages};
    use std::path::Path;

    // Parses the command-line options
    pub struct CommandLineParser {
        pub args: Vec<String>,
    }
    impl CommandLineParser {
        pub fn parse(&self) -> ParseResult {
            // Parse the arguments, and save the relevant options. Show messages for
            // any input errors encountered during parsing, and return True if the
            // parse failed

            // ParseResult configuration options
            let mut errors = false;

            // The available options
            let options = [
                String::from("-h"),
                String::from("--help")
            ];

            // Exit if there are not enough arguments
            if self.args.len() <4 {
                info_messages::help();
                return ParseResult { errors: true, help: false};
            }
            
            // Verify the paths
            let source = Path::new(&self.args[1]);
            let target = Path::new(&self.args[2]);

            // Raise errors if the paths don't exist
            if !source.exists() {
                error_messages::PathDoesNotExistError { path: source }.show();
                errors = true;
            }
            if !target.exists() {
                error_messages::PathDoesNotExistError { path: target }.show();
                errors = true;
            }
            
            // Verify the arguments
            for arg in &self.args[3..] {
                if !options.contains(arg) {
                    error_messages::UnknownOptionError { option: arg.to_string() }.show();
                }
            }

            // Return the parse result
            ParseResult { errors: errors, help: false }
        }
    }

    // The results of the parsing
    pub struct ParseResult {
        errors: bool,
        help: bool,
    }
}

pub fn extract(source: &Path, target: &Path) {
    // Extract the contents of SOURCE to TARGET
    println!("Extracting contents of {} to {}", source.display(), target.display());
}
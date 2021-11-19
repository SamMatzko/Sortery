use std::path::Path;

pub mod command_line {
    // Parse the data from the command line

    use super::super::messages::{info_messages};
    use std::path::Path;

    pub struct CommandLineParser {
        pub args: Vec<String>,
    }
    impl CommandLineParser {
        pub fn parse(&self) -> ParseResult {
            // Parse the arguments, and save the relevant options. Show messages for
            // any input errors encountered during parsing, and return True if the
            // parse failed

            // The available options
            let options = [
                String::from("-h"),
                String::from("--help")
            ];

            // Exit if there are not enough arguments
            if self.args.len() <3 {
                info_messages::help();
                return ParseResult { errors: true, help: false};
            }
            
            // Verify the paths
            let source = Path::new(&self.args[0]);
            let target = Path::new(&self.args[1]);
            
            // Verify the arguments
            for arg in &self.args {
                println!("{}", arg);
            }

            // Return the parse result
            ParseResult { errors: false, help: false }
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
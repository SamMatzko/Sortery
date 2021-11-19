use std::path::Path;

// Commmand-line tools
pub mod command_line {

    use super::super::messages::error_messages;
    use std::path::Path;

    // Parses the command-line options
    pub struct CommandLineParser {
        pub args: Vec<String>,
    }
    impl CommandLineParser {
        
        fn is_help_only(&self, args: &Vec<String>) -> bool {
            // Return true if the only command-line option was --help or -h

            if args.len() == 2 &&
            (args[1] == String::from("-h") || args[1] == String::from("--help"))
            {
                return true;
            } else { return false; }

        }

        pub fn parse(&self) -> ParseResult {
            // Parse the arguments, and save the relevant options. Show messages for
            // any input errors encountered during parsing, and return True if the
            // parse failed

            // ParseResult configuration options
            let mut errors = 0;
            let mut help = false;

            // The available options
            let options = [
                String::from("-h"),
                String::from("--help")
            ];

            // Check first if the user inputted for help
            if self.args.contains(&options[0]) || self.args.contains(&options[1]) {
                help = true;
            }

            // Show an error if there are not enough args, and --help isn't specified
            if self.args.len() <4 && !self.is_help_only(&self.args) {
                error_messages::NotEnoughArgsError {
                    len: self.args.len() - 1
                }.show();
                errors += 1;
                return ParseResult {
                    errors: errors,
                    help: help,
                    source: Path::new("<none>"),
                    target: Path::new("<none>"),
                };
            // If there were not enough args, but --help was specified, show
            // the help message.
            } else if self.is_help_only(&self.args) {
                return ParseResult {
                    errors: errors,
                    help: help,
                    source: Path::new("<none>"),
                    target: Path::new("<none>"),
                };
            }
            
            // Verify the paths
            let source = Path::new(&self.args[1]);
            let target = Path::new(&self.args[2]);

            // Raise errors if the paths don't exist
            if !source.exists() {
                error_messages::PathDoesNotExistError { path: source }.show();
                errors += 1;
            }
            if !target.exists() {
                error_messages::PathDoesNotExistError { path: target }.show();
                errors += 1;
            }
            
            // Verify the arguments
            for arg in &self.args[3..] {
                if !options.contains(arg) {
                    error_messages::UnknownOptionError {
                        option: arg.to_string()
                    }.show();
                    errors += 1;
                }
            }

            // Return the parse result
            ParseResult {
                errors: errors,
                help: help,
                source: source,
                target: target,
            }
        }
    }

    // The results of the parsing
    pub struct ParseResult <'a> {
        pub errors: i32,
        pub help: bool,
        pub source: &'a Path,
        pub target: &'a Path,
    }
}

pub fn extract(source: &Path, target: &Path) {
    // Extract the contents of SOURCE to TARGET
    println!("Extracting contents of {} to {}", source.display(), target.display());
}
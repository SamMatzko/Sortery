use super::messages::{error_messages, ProgressBar};
use std::{fs, path::Path};

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
            let mut extract = false;
            let mut sort = false;
            let mut help = false;

            // The available options
            let options = [
                String::from("-h"),
                String::from("--help"),
                String::from("-e"),
                String::from("--extract"),
                String::from("-s"),
                String::from("--sort"),
            ];

            // Check first if the user inputted for help
            if self.args.contains(&options[0]) || self.args.contains(&options[1]) {
                help = true;
            }

            // Show an error if there are not enough args, and --help isn't specified
            if self.args.len() <4 && !self.is_help_only(&self.args) {
                println!("{}", error_messages::NotEnoughArgsError {
                    len: self.args.len() - 1
                }.to_string());
                errors += 1;
                return ParseResult {
                    errors: errors,
                    help: help,
                    extract: extract,
                    sort: sort,
                    source: Path::new("<none>"),
                    target: Path::new("<none>"),
                };
            // If there were not enough args, but --help was specified, show
            // the help message.
            } else if self.is_help_only(&self.args) {
                return ParseResult {
                    errors: errors,
                    help: help,
                    extract: extract,
                    sort: sort,
                    source: Path::new("<none>"),
                    target: Path::new("<none>"),
                };
            }
            
            // Verify the paths
            let source = Path::new(&self.args[1]);
            let target = Path::new(&self.args[2]);

            // Raise errors if the paths don't exist
            if !source.exists() {
                println!("{}", error_messages::PathDoesNotExistError { path: source }.to_string());
                errors += 1;
            }
            if !target.exists() {
                println!("{}", error_messages::PathDoesNotExistError { path: target }.to_string());
                errors += 1;
            }
            
            // Verify the arguments
            for arg in &self.args[3..] {
                if !options.contains(arg) {
                    println!("{}", error_messages::UnknownOptionError {
                        option: arg.to_string()
                    }.to_string());
                    errors += 1;
                }
            }
            
            // Set the command options based on the command-line options

            // Exctract: -e, --extract
            if self.args.contains(&options[2]) || self.args.contains(&options[3]) {
                extract = true;
            }
            // Sort: -s, --sort
            if self.args.contains(&options[4]) || self.args.contains(&options[5]) {
                sort = true;
            }

            // Return the parse result
            ParseResult {
                errors: errors,
                extract: extract,
                sort: sort,
                help: help,
                source: source,
                target: target,
            }
        }
    }

    // The results of the parsing
    pub struct ParseResult <'a> {
        pub errors: i32,
        pub extract: bool,
        pub sort: bool,
        pub help: bool,
        pub source: &'a Path,
        pub target: &'a Path,
    }
}

// Various sorting algorithms
pub mod sort {

    // use super::super::messages::error_messages;
    use std::{path::Path};
    use walkdir::WalkDir;
    
    pub fn by_date(source: &Path, target: &Path) {
        // Sort all the files in SOURCE (including in all subdirs) by date into TARGET.
        // For now, this only works if SOURCE and TARGET are both outside each other.
        // Does not sort directories

        // The number of items we have sorted
        let mut items_sorted = 0;

        // Count the number of items we are going to sort
        let mut items_to_sort = 0;
        for entry in WalkDir::new(source.display().to_string()) {

            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {
                items_to_sort += 1;
            }
        }
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.display().to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The Path instance we are sorting
                let path = entry.path();

                println!("Sorting {}", path.display());
            }
        }
    }
}

pub fn extract(source: &Path, target: &Path) {
    // Extract the contents of SOURCE to TARGET

    // The number of items we have moved
    let mut items_moved = 0;

    // Count the number of items we are going to move
    let mut items_to_move = 0;
    for entry in source.read_dir().expect("Failed to read dir") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = entry.path();

        // Make sure that the path being moved is not the source or target
        if old_path == source || old_path == target { continue }

        items_to_move += 1;
    }

    // The progress bar
    let progress_bar = ProgressBar {
        completed_message: String::from("Completed."),
        message: String::from("Extracting..."),
        total: items_to_move,
    };

    // Move each entry (file or directory) in the directory
    for entry in source.read_dir().expect("Failed to read dir.") {

        // The entry path
        let entry = entry.expect("Failed to get dir entry.");
        let old_path = entry.path();

        // Calculate the new path for the entry
        let new_path = target.join(old_path.file_name().unwrap());

        // Make sure that the path being moved is not the source or target
        if old_path == source || old_path == target { continue }

        // Move the path
        // println!("Moving {} to {}...", &old_path.display(), &new_path.display());
        fs::rename(old_path.display().to_string(), new_path.display().to_string())
            .expect(
                &error_messages::PathMoveFailedError {
                    source: &old_path,
                    target: &new_path,
                }.to_string()
            );
        
        // Add to the count of items moved
        items_moved += 1;

        // Show the progress
        progress_bar.set_progress(items_moved);
    }
    // Show success status
    progress_bar.complete();
    println!("Successfully moved {} items to {}.", items_moved, target.display());
}
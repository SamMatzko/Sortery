use colored::Colorize;
use crate::structs::File;
use std::{io, io::Write};

// The highlighted message for dry-run output
pub struct DryRunMessage {
    pub from_file: File,
    pub to_file: File,
}
impl DryRunMessage {
    pub fn to_string(&self) -> String {
        String::from(format!(
            "Sorting {} to {}.",
            format!("{}", self.from_file.to_string()).green(),
            format!("{}", self.to_file.to_string()).red()
        ))
    }
}

// The progress bar used when sorting
pub struct ProgressBar {
    pub completed_message: String,
    pub message: String,
    pub total: usize,
}
impl ProgressBar {

    pub fn set_progress(&self, completed: usize) {
        // Print the progress bar

        let progress_done: usize = ((20f32/self.total as f32)*completed as f32) as usize;
        let progress_todo: usize = 20-progress_done as usize;

        print!(
            " {0} |{1}{2}| {3}% {4}/{5}{6}\r",
            self.message,
            "⌷".repeat(progress_done),
            "-".repeat(progress_todo),
            ((100f32/self.total as f32)*completed as f32) as usize,
            completed,
            self.total,
            " ".repeat(15)
        );
        io::stdout().flush().expect("Failed to flush stdout.");
    }

    pub fn complete(&self) {
        // Show the progress bar at completed status
        println!(
            "{0} |{1}| 100% {2}/{2}{3}",
            self.completed_message,
            "⌷".repeat(20),
            self.total,
            " ".repeat(15)
        )
    }
}

pub mod error_messages {
    // Error message templates for common errors
    
    use crate::structs::File;
    use colored::Colorize;

    // When the user inputs a file or directory that doesn't exist
    pub struct PathDoesNotExistError <'a> {
        pub path: &'a File,
    }
    impl <'a> PathDoesNotExistError <'a> {
        pub fn to_string(&self) -> String {
            // Return the error message as a string

            return format!(
                "{} no such file or directory \"{}\". Try sortery --help for more info.",
                format!("Error:").red(),
                self.path.to_string()
            );
        }
    }

    // When an attempt to move a path fails
    pub struct PathMoveFailedError <'a> {
        pub source: &'a File,
        pub target: &'a File,
    }
    impl <'a> PathMoveFailedError <'a> {
        pub fn to_string(&self) -> String {
            // Return the error message as a string

            return format!(
                "{} failed to move {} to {}.",
                    format!("Error:").red(),
                    self.source.to_string(),
                    self.target.to_string(),
                )
        }
    }
}
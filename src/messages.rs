//! The module containing all the commonly-used command-line messages.

use colored::Colorize;
use crate::structs::File;
use std::{io, io::Write};

/// The highlighted message for dry-run output.
pub struct DryRunMessage {
    pub from_file: File,
    pub to_file: File,
}
impl DryRunMessage {

    /// Return the highlighted message as a [`String`] that can be printed to the terminal
    pub fn to_string(&self) -> String {
        String::from(format!(
            "Sorting {} to {}.",
            format!("{}", self.from_file.to_string()).green(),
            format!("{}", self.to_file.to_string()).red()
        ))
    }
}

/// The command-line progress bar used when sorting.
/// 
/// <ul>
/// <li>
/// 
/// `completed_message` is a [`String`] of the message to be printed when
/// [`ProgressBar::complete`] is called.
/// </li>
/// <li>
/// 
/// `message` is a [`String`] to be shown while the progress bar is between start
/// and finish, for example `"Still working..."`
/// 
/// `total` is a [`usize`] representing the total value of the progress bar. Bar
/// pogress and percent completed are calculated using `total`. For example, if
/// you are going to sort 20 files, you would pass 20 to `total`, and the progress
/// bar would know to show `50%` when `set_progress(10)` is called.
pub struct ProgressBar {
    pub completed_message: String,
    pub message: String,
    pub total: usize,
}
impl ProgressBar {

    /// Print the updated progress bar, with `completed` number of items completed
    /// out of the total. Automatically calculates percent and bar size.
    pub fn set_progress(&self, completed: usize) {

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

    /// Print the full progress bar, along with `completed_message`.
    pub fn complete(&self) {
        println!(
            "{0} |{1}| 100% {2}/{2}{3}",
            self.completed_message,
            "⌷".repeat(20),
            self.total,
            " ".repeat(15)
        )
    }
}

/// Error message structs for common errors.
pub mod error_messages {
    
    use crate::structs::File;
    use colored::Colorize;

    /// When the user inputs a file or directory that doesn't exist. `path` is
    /// a [`File`] representing the path that does not exist.
    pub struct PathDoesNotExistError <'a> {
        pub path: &'a File,
    }
    impl <'a> PathDoesNotExistError <'a> {

        /// Return the full, colorized error message as a string.
        pub fn to_string(&self) -> String {

            return format!(
                "{} no such file or directory \"{}\". Try sortery --help for more info.",
                format!("Error:").red(),
                self.path.to_string()
            );
        }
    }

    /// When an attemtped file rename fails. [`File`] `source` is the old file path,
    /// and [`File`] `target` is the path that `source` should have been renamed to.
    pub struct PathMoveFailedError <'a> {
        pub source: &'a File,
        pub target: &'a File,
    }
    impl <'a> PathMoveFailedError <'a> {

        /// Return the full, colorized error message as a string.
        pub fn to_string(&self) -> String {

            return format!(
                "{} failed to move {} to {}.",
                    format!("Error:").red(),
                    self.source.to_string(),
                    self.target.to_string(),
                )
        }
    }
}
mod messages;
mod tools;

use clap::{Arg, App};
use messages::error_messages;
use std::path::Path;

fn main() {

    // Some of the text used in the app creation
    let about = "Sortery is a basic file sorter.";
    let extract_help = "Move all files and directories from SOURCE to TARGET";

    // Get the command-line arguments using clap::App
    let matches = App::new("Sortery")
                        .version("1.0.0")
                        .author("Samuel Matzko")
                        .about(about)
                        .arg(Arg::with_name("SOURCE")
                            .help("The source directory")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("TARGET")
                            .help("The target directory.")
                            .required(true)
                            .index(2))
                        .arg(Arg::with_name("extract")
                            .short("e")
                            .long("extract")
                            .help(extract_help))
                        .arg(Arg::with_name("by-date")
                            .long("by-date")
                            .help("Sort all files from SOURCE into TARGET by date"))
                        .get_matches();
    /*
    Run everything according to the command-line arguments
    */

    // The variable telling whether we need to exit because of an error
    let exit_for_error = false;

    // The source and target directories
    let source = Path::new(matches.value_of("SOURCE").unwrap());
    let target = Path::new(matches.value_of("TARGET").unwrap());

    // Check the existence of source and target direcotories, and raise errors
    // if they don't exist
    if !source.exists() {
        println!("{}", error_messages::PathDoesNotExistError { path: &source }.to_string());
    }
    if !target.exists() {
        println!("{}", error_messages::PathDoesNotExistError { path: &target }.to_string());
    }

    // Exit if there were any errors
    if exit_for_error { return; }
    
    // Run the commands
    if matches.is_present("extract") {
        tools::extract(source, target);
    } else if matches.is_present("by-date") {
        tools::sort::by_date(source, target);
    }
}
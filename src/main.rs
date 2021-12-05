mod messages;
mod tools;

use clap::{Arg, App};
use std::path::Path;

fn main() {

    // Some of the text used in the app creation
    let about = "Coral is a basic file and directory sorter.";
    let extract_help = "Move all files and directories from SOURCE to TARGET";

    // Get the command-line arguments using clap::App
    let matches = App::new("Coral")
                        .version("0.2.3")
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

    // The source and target directories
    let source = Path::new(matches.value_of("SOURCE").unwrap());
    let target = Path::new(matches.value_of("TARGET").unwrap());
    
    // Run the commands
    if matches.is_present("extract") {
        tools::extract(source, target);
    } else if matches.is_present("by-date") {
        tools::sort::by_date(source, target);
    }
}
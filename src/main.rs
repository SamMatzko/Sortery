mod messages;
mod tools;

use clap::{App, Arg, SubCommand};
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
                            .help("The source directory.")
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
                        .subcommand(SubCommand::with_name("sort")
                            .about("Advanced sorting options.")
                            .arg(Arg::with_name("preserve-name")
                                .short("p")
                                .long("preserve-name")
                                .help("Preserve the original file name when renaming."))
                            .arg(Arg::with_name("date-format")
                                .long("date-format")
                                .takes_value(true)
                                .default_value("%Y-%m-%d %Hh%Mm%Ss")
                                .help("The date format for renaming files."))
                            .arg(Arg::with_name("date-type")
                                .long("date-type")
                                .takes_value(true)
                                .default_value("c")
                                .help("Set which date to sort by.")))
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
    } else if matches.is_present("sort") {

        // The sub-command matches
        let sub_matches = matches.subcommand_matches("sort").unwrap();

        // Variables configured by the command-line options and used when
        // running the sort tool.
        let date_format = sub_matches.value_of("date-format").unwrap_or("%Y-%m-%d %Hh%Mm%Ss");
        let date_type = sub_matches.value_of("date-type").unwrap_or("c");
        let preserve_name: bool = sub_matches.is_present("preserve-name");

        // Run the sort tool
        tools::sort::sort(source, target, &date_format, &date_type, &preserve_name);
    }
}
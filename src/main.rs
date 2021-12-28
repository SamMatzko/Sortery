//! Sortery is a simple, fast command-line file sorter for Linux.
//! You can find the GitHib repo at <https://github.com/SamMatzko/Sortery>, and the
//! Sortery Wiki at <https://github.com/SamMatzko/Sortery/wiki>.

mod messages;
mod structs;
mod tools;

use clap::{App, Arg, SubCommand};
use messages::error_messages;
use std::fs;
use structs::File;

/// Run the whole Sortery application, including the cli.
fn main() {

    // Some of the text used in the app creation
    let about = "\nSortery is a simple, fast command-line file sorter for Linux. \
You can find the GitHib repo at <https://github.com/SamMatzko/Sortery>, and the \
Sortery Wiki at <https://github.com/SamMatzko/Sortery/wiki>.";
    let config_help = "Path to a JSON file with configuration information. Use template.json as a template.";
    let extract_help = "Move all files and directories from SOURCE to TARGET";

    // Get the command-line arguments using clap::App
    let matches = App::new("Sortery")
                        .version("1.4.2")
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
                        .arg(Arg::with_name("config-file")
                            .short("c")
                            .long("config-file")
                            .takes_value(true)
                            .help(config_help))
                        .arg(Arg::with_name("extract")
                            .short("e")
                            .long("extract")
                            .help(extract_help))
                        .arg(Arg::with_name("dry-run")
                            .short("d")
                            .long("dry-run")
                            .help("Show the intended sort, without actually sorting."))
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
                                .help("Set which date to sort by."))
                            .arg(Arg::with_name("exclude-type")
                                .short("e")
                                .long("exclude-type")
                                .takes_value(true)
                                .help("File extension(s) to exclude when sorting."))
                            .arg(Arg::with_name("only-type")
                                .short("o")
                                .long("only-type")
                                .takes_value(true)
                                .help("File extension(s) to exclusively sort.")))
                        .get_matches();
    /*
    Run everything according to the command-line arguments
    */

    // The variable telling whether we need to exit because of an error
    let mut exit_for_error = false;

    // The source and target directories
    let source = File::new(matches.value_of("SOURCE").unwrap());
    let target = File::new(matches.value_of("TARGET").unwrap());

    // Check the existence of source and target direcotories, and raise errors
    // if they don't exist
    if !source.exists() {
        println!("{}", error_messages::PathDoesNotExistError { path: &source }.to_string());
        exit_for_error = true;
    }
    if !target.exists() {
        println!("{}", error_messages::PathDoesNotExistError { path: &target }.to_string());
        exit_for_error = true;
    }

    // Exit if there were any errors
    if exit_for_error { return; }

    // If a json config file was given, sort according to it
    if matches.is_present("config-file") {
        tools::sort::sort_from_json(
            fs::read_to_string(matches.value_of("config-file").unwrap())
                .expect("Failed to read config file."),
            source,
            target,
            matches.is_present("dry-run")
        );
        return;
    }
    
    // Run the commands
    if matches.is_present("extract") {
        tools::extract(&source, &target, matches.is_present("dry-run"));
    } else if matches.is_present("sort") {

        // The sub-command matches
        let sub_matches = matches.subcommand_matches("sort").unwrap();

        // Variables configured by the command-line options and used when
        // running the sort tool.
        let date_format = sub_matches.value_of("date-format").unwrap_or("%Y-%m-%d %Hh%Mm%Ss");
        let date_type = sub_matches.value_of("date-type").unwrap_or("c");
        let preserve_name: bool = sub_matches.is_present("preserve-name");
        let exclude_type = (
            sub_matches.value_of("exclude-type").unwrap_or(""),
            sub_matches.is_present("exclude-type")
        );
        let only_type = (
            sub_matches.value_of("only-type").unwrap_or(""),
            sub_matches.is_present("only-type")
        );

        // Run the sort tool, or dry run if commanded
        tools::sort::sort(
            &source,
            &target,
            &date_format,
            &date_type,
            &preserve_name,
            exclude_type,
            only_type,
            matches.is_present("dry-run")
        );
    }
}
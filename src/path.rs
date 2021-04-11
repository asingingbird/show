use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use log::error;
use std::env;
use std::path::{Path, PathBuf};

pub struct PathCommand;

impl UtilSubCommand for PathCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("path")
            .version("1.0")
            .about("Show the path of a file or directory")
            .arg(
                Arg::with_name("path")
                    .value_name("PATH")
                    .required_unless("temp_dir")
                    .help("Print the path of this file or directory"),
            )
            .arg(
                Arg::with_name("temp_dir")
                    .long("temp")
                    .short("t")
                    .conflicts_with("path")
                    .help("Print temp directory path"),
            )
            .arg(
                Arg::with_name("fullname")
                    .long("full-name")
                    .short("F")
                    .help(" Prints the full path prefix for each file"),
            )
            .arg(
                Arg::with_name("absolute")
                    .long("absolute")
                    .short("A")
                    .help(" Prints the absolute path for each file"),
            )
            .arg(
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help(" Prints all entries, including hidden files"),
            )
            .arg(
                Arg::with_name("recursive")
                    .long("recursive")
                    .short("r")
                    .help("Print subdirectories recursively"),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_path(args);
    }
}

fn run_path(args: &ArgMatches) {
    if args.is_present("temp_dir") {
        print_path(&env::temp_dir());
        return;
    }

    args.value_of("path").map_or_else(
        || error!("No file or directory specified, run 'show path --help' for usage"),
        |p| print_path(Path::new(p)),
    );
}

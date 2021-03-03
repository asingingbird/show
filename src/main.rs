mod diff;
mod path;

use path::*;
use log::{debug, error, info, trace, warn};

use crate::diff::{diff_sub_command, run_diff};
use clap::{App, ArgMatches};

fn main() {
    let matches = App::new("show")
        .author("asingingbird.cb")
        .version("1.0")
        .about("Display magic things on your screen")
        .subcommand(path_sub_command())
        .subcommand(diff_sub_command())
        .get_matches();

    match matches.subcommand() {
        ("path", Some(m)) => run_path(m),
        ("diff", Some(m)) => run_diff(m),
        _ => {}
    }
}

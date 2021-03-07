use super::utils::UtilSubCommand;
use clap::{App, Arg, ArgMatches, SubCommand};
use log::warn;

pub struct DiffCommand;

impl UtilSubCommand for DiffCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("diff")
            .about("show the difference of two files")
            .version("1.0")
            .arg(
                Arg::with_name("bytes")
                    .long("bytes")
                    .short("b")
                    .conflicts_with_all(&["lines", "chars"])
                    .help("Show difference in bytes"),
            )
            .arg(
                Arg::with_name("lines")
                    .long("lines")
                    .short("l")
                    .help("Show difference in lines"),
            )
            .arg(
                Arg::with_name("chars")
                    .long("chars")
                    .short("c")
                    .help("Show difference in chars (default)"),
            )
            .arg(
                Arg::with_name("left_file")
                    .value_name("LEFT_FILE")
                    .help("The left file to diff"),
            )
            .arg(
                Arg::with_name("right_file")
                    .value_name("RIGHT_FILE")
                    .help("The right file to diff"),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_diff(args);
    }
}

fn run_diff(args: &ArgMatches) {
    let left_file = args.value_of("left_file");
    let right_file = args.value_of("right_file");
    if left_file.is_none() {
        warn!("No left file specified");
    }
    if right_file.is_none() {
        warn!("No right file specified");
    }
}

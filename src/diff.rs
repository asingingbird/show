use clap::{App, Arg, ArgMatches, SubCommand};

pub fn diff_sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("diff")
        .about("show the difference of two files")
        .version("1.0")
        .arg(
            Arg::with_name("left_file")
                .value_name("LEFT_FILE")
                .index(1)
                .required(true)
                .help("the left file to diff"),
        )
        .arg(
            Arg::with_name("right_file")
                .value_name("RIGHT_FILE")
                .index(2)
                .required(true)
                .help("the right file to diff"),
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("b")
                .conflicts_with_all(&["lines", "chars"])
                .help("show difference in bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .short("l")
                .help("show difference in lines"),
        )
        .arg(
            Arg::with_name("chars")
                .long("chars")
                .short("c")
                .help("show difference in chars (default)"),
        )
}

pub fn run_diff(args: &ArgMatches) {}

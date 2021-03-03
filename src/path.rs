use clap::{App, Arg, ArgMatches, SubCommand};

pub fn path_sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("path")
        .about("show the path of a file or directory")
        .version("1.0")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .index(1)
                .required(true)
                .help("the path to show"),
        )
        .arg(
            Arg::with_name("absolute")
                .long("absolute")
                .short("a")
                .help("only print absolute path"),
        )
        .arg(
            Arg::with_name("relative")
                .long("relative")
                .short("r")
                .help("only print relative path"),
        )
}

pub fn run_path(args: &ArgMatches) {}

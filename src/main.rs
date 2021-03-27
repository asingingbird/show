mod completion;
mod diff;
mod path;
mod utils;
mod which;

use diff::*;
use path::*;
use which::*;
use completion::*;
use utils::*;

use clap::App;
use colored::Colorize;
use fern::colors::ColoredLevelConfig;

fn setup_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "\x1b[{}m{}: {}\x1b[0m",
                ColoredLevelConfig::new()
                    .get_color(&record.level())
                    .to_fg_str(),
                record.level().to_string().to_lowercase(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap_or_else(|e| {
            println!(
                "{}",
                format!("Setup logger failed: {}.", e.to_string()).red()
            )
        });
}

pub fn build_app() -> App<'static, 'static> {
    App::new("show")
        .author("asingingbird.cb")
        .version("1.0")
        .about("Show some magic things")
        .subcommand(WhichCommand::util_sub_command())
        .subcommand(PathCommand::util_sub_command())
        .subcommand(DiffCommand::util_sub_command())
        .subcommand(CompletionCommand::util_sub_command())
}

fn main() {
    setup_logger();

    let matches = build_app().get_matches();

    match matches.subcommand() {
        ("which", Some(m)) => WhichCommand::run(m),
        ("path", Some(m)) => PathCommand::run(m),
        ("diff", Some(m)) => DiffCommand::run(m),
        ("generate-completions", Some(m)) => CompletionCommand::run(m),
        _ => {}
    }
}

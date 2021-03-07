mod cli;
mod completion;
mod diff;
mod path;
mod utils;

use cli::*;
use path::*;

use crate::completion::CompletionCommand;
use colored::Colorize;
use diff::*;
use fern::colors::ColoredLevelConfig;
use utils::UtilSubCommand;

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

fn main() {
    setup_logger();

    let matches = build_cli().get_matches();

    match matches.subcommand() {
        ("path", Some(m)) => PathCommand::run(m),
        ("diff", Some(m)) => DiffCommand::run(m),
        ("generate-completions", Some(m)) => CompletionCommand::run(m),
        _ => {}
    }
}

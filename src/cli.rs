use crate::completion::CompletionCommand;
use crate::diff::DiffCommand;
use crate::path::PathCommand;
use crate::utils::UtilSubCommand;
use clap::App;

pub fn build_cli() -> App<'static, 'static> {
    App::new("show")
        .author("asingingbird.cb")
        .version("1.0")
        .about("Display magic things on your screen")
        .subcommand(PathCommand::util_sub_command())
        .subcommand(DiffCommand::util_sub_command())
        .subcommand(CompletionCommand::util_sub_command())
}

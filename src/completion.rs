use super::utils::UtilSubCommand;
use crate::build_app;
use clap::{App, Arg, ArgMatches, Shell, SubCommand};
use log::error;
use std::fs::File;

pub struct CompletionCommand;

impl UtilSubCommand for CompletionCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("generate-completions")
            .about("Generate a completions file")
            .version("1.0")
            .arg(
                Arg::with_name("shell")
                    .long("shell")
                    .short("s")
                    .value_name("SHELL")
                    .takes_value(true)
                    .required(true)
                    .possible_values(&["bash", "fish", "zsh", "powershell", "elvish"])
                    .help("Which shell to produce a completions file for"),
            )
            .arg(
                Arg::with_name("output")
                    .long("out")
                    .short("o")
                    .takes_value(true)
                    .help(
                        "A file to output the contents of the completion script,\
                     use stdout if not provided",
                    ),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_gen_completion(args)
    }
}

fn run_gen_completion(args: &ArgMatches) {
    if let Some(sh) = args.value_of("shell") {
        let shell = match sh {
            "bash" => Shell::Bash,
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            "powershell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => {
                error!("Undefined shell type");
                return;
            }
        };

        if let Some(f) = args.value_of("output") {
            let mut output_file = match File::create(f) {
                Ok(file) => file,
                Err(e) => {
                    error!("{}: {}", f, e.to_string());
                    return;
                }
            };
            build_app().gen_completions_to("show", shell, &mut output_file);
        } else {
            build_app().gen_completions_to("show", shell, &mut std::io::stdout());
        }
    };
}

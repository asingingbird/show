use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use std::path::PathBuf;

pub struct WhichCommand;

impl UtilSubCommand for WhichCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("which")
            .about("Print the path of the executable file")
            .version("1.0")
            .arg(
                Arg::with_name("bin")
                    .value_name("BIN")
                    .takes_value(true)
                    .required(true)
                    .help("Print the path of the executable file"),
            )
            .arg(
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help("Print all possible paths"),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_which(args);
    }
}

fn executable_candidates(exe: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut sys_paths = Vec::new();

    if let Some(p) = env::var_os("PATH") {
        sys_paths.extend(env::split_paths(&p));
    }

    let binary_lowercase = exe.to_ascii_lowercase();

    #[cfg(windows)]
    {
        let binary = PathBuf::from(exe);
        if binary.extension().is_none() {
            if let Some(ext) = env::var_os("PATHEXT") {
                let binaries_with_extension = env::split_paths(&ext)
                    .map(|extension| {
                        let mut bin = binary_lowercase.clone();
                        bin.push_str(&extension.to_string_lossy().to_ascii_lowercase());
                        bin
                    })
                    .collect::<Vec<_>>();

                candidates.extend(sys_paths.iter().flat_map(|path| {
                    binaries_with_extension.iter().map(move |bin| {
                        let mut binary_path = path.clone();
                        binary_path.push(&bin);
                        binary_path
                    })
                }));
            }
        }
    }

    candidates.extend(sys_paths.into_iter().map(|mut candidate| {
        candidate.push(binary_lowercase.clone());
        candidate
    }));

    candidates
}

fn search_executable(exe: &str) -> Option<PathBuf> {
    executable_candidates(exe)
        .iter()
        .find(|c| c.is_executable())
        .cloned()
}

fn search_all_executables(exe: &str) -> Vec<PathBuf> {
    executable_candidates(exe)
        .iter()
        .filter(|c| c.is_executable())
        .cloned()
        .collect()
}

fn run_which(args: &ArgMatches) {
    if let Some(b) = args.value_of("bin") {
        if args.is_present("all") {
            for bin in search_all_executables(b) {
                print_path(&bin);
            }
        } else if let Some(bin) = search_executable(b) {
            print_path(&bin);
        }
    }
}

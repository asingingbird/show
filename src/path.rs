use super::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;
use log::error;
use std::env;
use std::path::{Path, PathBuf};

pub struct PathCommand;

impl UtilSubCommand for PathCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        let mut cmd = SubCommand::with_name("path")
            .about("show the path of a file or directory")
            .version("1.0")
            .arg(
                Arg::with_name("path")
                    .value_name("PATH")
                    .required_unless_one(&["bin", "temp_dir"])
                    .help("Print the absolute path of this file or directory"),
            )
            .arg(
                Arg::with_name("bin")
                    .long("bin")
                    .short("b")
                    .value_name("BIN")
                    .takes_value(true)
                    .conflicts_with("path")
                    .help("Print the path of the executable file"),
            )
            .arg(
                Arg::with_name("temp_dir")
                    .long("temp")
                    .short("t")
                    .conflicts_with_all(&["path", "bin"])
                    .help("Print temp directory path"),
            )
            .arg(
                Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .help("Print all possible paths"),
            );
        #[cfg(windows)]
        {
            cmd = cmd.arg(
                Arg::with_name("unix")
                    .long("unix")
                    .short("u")
                    .help("Print path in unix style"),
            );
        }
        cmd
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_path(args);
    }
}

fn print_path(path: &Path, use_unix_style: bool) {
    let absolute_path = match path.to_absolute() {
        Ok(p) => p,
        Err(e) => {
            error!("Get current directory failed: {}", e.to_string());
            return;
        }
    };

    if !absolute_path.exists() {
        error!("No such file or directory: {:?}", absolute_path);
        return;
    }

    let path_string = if use_unix_style {
        absolute_path.to_unix_style()
    } else {
        absolute_path.to_string_lossy().to_string()
    };

    print!("{}", path_string);

    if absolute_path.is_symlink() {
        let link_to = absolute_path.read_link().unwrap();
        let colored_link = if link_to.exists() {
            link_to.to_string_lossy().to_string().green()
        } else {
            link_to.to_string_lossy().to_string().red()
        };
        println!(" {} {}", "-->".cyan().bold(), colored_link);
    } else {
        println!();
    };
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

fn run_path(args: &ArgMatches) {
    let use_unix_style = args.is_present("unix");

    if args.is_present("temp_dir") {
        print_path(&env::temp_dir(), use_unix_style);
        return;
    }

    if let Some(b) = args.value_of("bin") {
        if args.is_present("all") {
            for bin in search_all_executables(b) {
                print_path(&bin, use_unix_style);
            }
        } else if let Some(bin) = search_executable(b) {
            print_path(&bin, use_unix_style);
        }
        return;
    }

    args.value_of("path").map_or_else(
        || error!("No file or directory specified, run 'show path --help' for usage"),
        |p| print_path(Path::new(p), use_unix_style),
    );
}

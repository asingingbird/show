use super::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;
use log::error;
use std::env;
use std::path::{Path, PathBuf};

pub struct PathCommand;

impl UtilSubCommand for PathCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("path")
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
            )
            .arg(
                Arg::with_name("unix")
                    .long("unix")
                    .short("u")
                    .help("Print path in unix style, default on unix os"),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_path(args);
    }
}

fn print_path(path: &Path, use_unix_style: bool) {
    let cwd = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            error!("Get current directory failed: {}", e.to_string());
            return;
        }
    };
    let absolute_path = path.to_absolute(&cwd);

    // Do not follow symlink here
    if absolute_path.symlink_metadata().is_err() {
        error!("No such file or directory: {:?}", absolute_path);
        return;
    }

    let path_string = if use_unix_style && cfg!(windows) {
        absolute_path.to_unix_style()
    } else {
        absolute_path.to_string_lossy().to_string()
    };

    if absolute_path.is_symlink() {
        let symlink = absolute_path.read_link().unwrap();

        let mut link_to = absolute_path;
        link_to.pop();
        link_to = symlink.to_absolute(&link_to);
        let colored_link = if link_to.exists() {
            link_to.to_string_lossy().to_string().green()
        } else {
            link_to.to_string_lossy().to_string().red()
        };
        println!("{} {} {}", path_string, "-->".cyan().bold(), colored_link);
    } else {
        println!("{}", path_string);
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

use super::utils::UtilSubCommand;
use clap::{App, Arg, ArgMatches, SubCommand};
use ignore::{Walk, WalkBuilder, WalkParallel};
use jwalk::WalkDir;
use log::warn;
use std::path::PathBuf;

pub struct LocationCommand;

impl UtilSubCommand for LocationCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("location")
            .about("Search for files in a directory")
            .version("1.0")
            .arg(
                Arg::with_name("file_type")
                    .long("type")
                    .short("t")
                    .possible_values(&[
                        "d",
                        "directory",
                        "e",
                        "empty",
                        "f",
                        "file",
                        "l",
                        "symlink",
                        "p",
                        "pipe",
                        "s",
                        "socket",
                        "x",
                        "executable",
                    ])
                    .hide_possible_values(true)
                    .help("Search by file type"),
            )
            .arg(
                Arg::with_name("extension")
                    .long("extension")
                    .short("e")
                    .help("Search by file extension"),
            )
            .arg(
                Arg::with_name("case_insensitive")
                    .long("ignore_case")
                    .short("i")
                    .help("Perform a case-insensitive search"),
            )
            .arg(
                Arg::with_name("search_pattern")
                    .value_name("PATTERN")
                    .help("The search pattern"),
            )
            .arg(
                Arg::with_name("search_path")
                    .value_name("DIRECTORY")
                    .help("The directory to search"),
            )
    }

    #[inline]
    fn run(args: &ArgMatches) {
        run_location();
    }
}

pub fn walk1(dir: &str) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    let walk = WalkBuilder::new(dir).git_ignore(false).build();

    for path in walk {
        vec.push(path.unwrap().path().to_path_buf());
    }
    vec
}

pub fn walk2(dir: &str) -> Vec<PathBuf> {
    let walk_parallel = WalkBuilder::new(dir).git_ignore(false).build_parallel();
    let (tx, rx) = std::sync::mpsc::channel();
    let collect_thread = std::thread::spawn(move || {
        let mut vec = Vec::new();
        for path in rx {
            vec.push(path);
        }
        vec
    });

    walk_parallel.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            let r = result.unwrap().path().to_path_buf();
            tx.send(r);
            ignore::WalkState::Continue
        })
    });
    drop(tx);
    collect_thread.join().unwrap()
}

pub fn walk3(dir: &str) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    let walk = WalkDir::new(dir).sort(true);

    for path in walk {
        vec.push(path.unwrap().path().to_path_buf());
    }
    vec
}

pub fn walk4(dir: &str) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    let walk = WalkDir::new(dir);

    for path in walk {
        vec.push(path.unwrap().path().to_path_buf());
    }
    vec
}

pub fn run_location() {
    let dir = r"C:\Users\asing\Desktop";
    let walk = WalkDir::new(dir).sort(true);

    let mut cnt = 0;
    for path in walk {
        cnt += 1;
        println!("{:?}", path.unwrap().path().to_path_buf());
    }
    println!("cnt = {}", cnt);
    return;
    let mut v1 = walk1(&dir);
    let mut v2 = walk2(&dir);
    let mut v3 = walk3(&dir);
    let mut v4 = walk4(&dir);
    if v1 == v3 {
        println!("YES");
    } else {
        println!("NO");
    }

    v1.sort();
    v2.sort();
    v4.sort();
    if v1 == v2 && v2 == v3 && v3 == v4 {
        println!("OK");
    } else {
        println!("NO");
    }
    println!("v1: {:?}\nv2: {:?}\nv3: {:?}\nv4: {:?}\n", v1, v2, v3, v4);
}

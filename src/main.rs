use clap::{command, Arg, ArgAction, Command};
use std::{
    cmp::Ordering,
    fs::{self},
    io::{self},
    path,
};

mod parse;

static FLAG_FILE: &str = "file";
static FLAG_REVERSE: &str = "reverse";

struct SortSettings {
    ascend: bool,
}

fn main() {
    let cli_matches = cli().get_matches();

    let mut file_paths: Option<Vec<&String>> = None;
    if let Some(file_arg_matches) = cli_matches.get_many::<String>(FLAG_FILE) {
        file_paths = Some(file_arg_matches.collect());
    }

    let settings = SortSettings {
        ascend: !cli_matches.get_flag(FLAG_REVERSE),
    };
    match file_paths {
        Some(paths) => sort_files(&settings, paths),
        None => sort_stdin(&settings),
    }
}

fn cli() -> Command {
    command!()
        // .arg(
        //     Arg::new("check")
        //     .short('c')
        //     .action(ArgAction::SetTrue)
        // )
        .arg(Arg::new(FLAG_REVERSE).short('r').action(ArgAction::SetTrue))
        .arg(Arg::new(FLAG_FILE).num_args(0..))
}

fn sort_files(settings: &SortSettings, file_paths: Vec<&String>) {
    let mut line_accumulator: Vec<String> = Vec::new();

    let paths = file_paths.iter().map(|path_str| path::Path::new(path_str));
    for path in paths {
        match path.try_exists() {
            Ok(exists) if (exists && path.is_dir()) => {
                eprintln!("sort: Is a directory");
                std::process::exit(2);
            }
            Ok(exists) if !exists => {
                eprintln!("sort: No such file or directory");
                std::process::exit(2);
            }
            Ok(_) => {
                if let Ok(content) = fs::read_to_string(path) {
                    parse::tokenize_line(&mut line_accumulator, &content);
                } else {
                    eprintln!("sort: Error when reading file {:?}", path);
                    std::process::exit(1);
                }
            }
            Err(_) => {
                eprintln!("sort: unknown error");
                std::process::exit(1);
            }
        }
    }

    line_accumulator.sort_by(|a, b| line_order(settings, a, b));

    for string in line_accumulator.iter() {
        println!("{}", string);
    }
}

fn sort_stdin(settings: &SortSettings) {
    let mut line_accumulator: Vec<String> = Vec::new();
    for input_line in io::stdin().lines() {
        match input_line {
            Ok(line_string) => parse::tokenize_line(&mut line_accumulator, &line_string),
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
    }

    line_accumulator.sort_by(|a, b| line_order(settings, a, b));

    for string in line_accumulator.iter() {
        println!("{}", string);
    }
}

fn line_order(settings: &SortSettings, first_line: &String, second_line: &String) -> Ordering {
    return match first_line.cmp(second_line) {
        Ordering::Greater => {
            if settings.ascend {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Ordering::Equal => Ordering::Equal,
        Ordering::Less => {
            if settings.ascend {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    };
}

// fn check_sorted(settings: &SortSettings, lines: Vec<&String>) {
//     let mut prev: Option<&String> = None;
//     for line in lines {
//         match prev {
//             Some(prev_line) => {
//                 match line_order(settings, prev_line, line) {
//                     Ordering::Equal => continue,
//                     _ => {

//                     }
//                 };
//             },
//             None => prev = Some(line)
//         }
//     }
// }

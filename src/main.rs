use clap::{command, Arg, ArgAction, Command};
use std::{
    cmp::Ordering,
    fs::{self},
    io::{self},
    iter::{self},
    path,
};

mod parse;

const FLAG_CHECK: &str = "check";
const FLAG_REVERSE: &str = "reverse";
const FLAG_UNIQUE: &str = "unique";

const UNNAMED_ARGS: &str = "file";

const STDIN_KEYWORD: &str = "-";

struct SortSettings {
    ascend: bool,
    checked_file_name: Option<String>,
    unique: bool,
}

enum SortInput<'a> {
    File { path: &'a path::Path },
    Stdin,
}

fn main() {
    let cli_matches = cli().get_matches();

    let unnamed_args = match cli_matches.get_many::<String>(UNNAMED_ARGS) {
        Some(unnamed_arg_matches) => unnamed_arg_matches.collect(),
        None => Vec::new(),
    };

    let settings = SortSettings {
        ascend: !cli_matches.get_flag(FLAG_REVERSE),
        checked_file_name: match cli_matches.get_flag(FLAG_CHECK) {
            true if unnamed_args.is_empty() => STDIN_KEYWORD.to_string().into(),
            true => Some(unnamed_args[0].clone()),
            false => None,
        },
        unique: cli_matches.get_flag(FLAG_UNIQUE),
    };

    let sort_inputs = match unnamed_args.as_slice() {
        [] => vec![SortInput::Stdin],
        [first] => vec![path_arg_to_sort_input(&first)],
        [first, rest @ ..] => {
            if settings.checked_file_name.is_some() {
                vec![path_arg_to_sort_input(&first)]
            } else {
                iter::once(first)
                    .chain(rest.iter())
                    .map(|input| path_arg_to_sort_input(input))
                    .collect()
            }
        }
    };

    let mut line_accumulator: Vec<String> = Vec::new();

    accumulate_lines(sort_inputs, &mut line_accumulator);

    if settings.checked_file_name.is_some() {
        check_sorted(&settings, &line_accumulator);
    } else {
        sort_all(&settings, &mut line_accumulator);
    }
}

fn path_arg_to_sort_input(path: &String) -> SortInput {
    if path.as_str() == STDIN_KEYWORD {
        SortInput::Stdin
    } else {
        SortInput::File {
            path: path::Path::new(path),
        }
    }
}

fn cli() -> Command {
    command!()
        .arg(Arg::new(FLAG_CHECK).short('c').action(ArgAction::SetTrue))
        .arg(Arg::new(FLAG_REVERSE).short('r').action(ArgAction::SetTrue))
        .arg(Arg::new(FLAG_UNIQUE).short('u').action(ArgAction::SetTrue))
        .arg(Arg::new(UNNAMED_ARGS).num_args(0..))
}

fn accumulate_lines(sort_inputs: Vec<SortInput<'_>>, line_accumulator: &mut Vec<String>) {
    for input in sort_inputs {
        match input {
            SortInput::File { path } if path.exists() && path.is_dir() => {
                eprintln!("sort: Is a directory");
                std::process::exit(2);
            }
            SortInput::File { path } if !path.exists() => {
                eprintln!("sort: No such file or directory");
                std::process::exit(2);
            }
            SortInput::File { path } => {
                if let Ok(content) = fs::read_to_string(path) {
                    parse::tokenize_line(line_accumulator, &content);
                } else {
                    eprintln!("sort: Error when reading file {:?}", path);
                    std::process::exit(1);
                }
            }
            SortInput::Stdin => {
                for input_line in io::stdin().lines() {
                    match input_line {
                        Ok(line_string) => parse::tokenize_line(line_accumulator, &line_string),
                        Err(error) => {
                            eprintln!("Error: {}", error);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
}

fn line_order(settings: &SortSettings, first_line: &String, second_line: &String) -> Ordering {
    match first_line.cmp(second_line) {
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
    }
}

fn sort_all(settings: &SortSettings, lines: &mut Vec<String>) {
    lines.sort_by(|a, b| line_order(settings, a, b));
    if settings.unique {
        lines.dedup();
    }
    for string in lines.iter() {
        println!("{}", string);
    }
}

fn check_sorted(settings: &SortSettings, lines: &Vec<String>) {
    let mut prev: Option<String> = None;
    for (idx, line) in lines.iter().enumerate() {
        if let Some(ref prev_line) = prev {
            match line_order(settings, &prev_line, &line) {
                Ordering::Greater => {
                    eprintln!(
                        "sort: {}:{}: disorder: {}",
                        settings.checked_file_name.as_ref().unwrap(),
                        idx + 1,
                        line
                    );
                    std::process::exit(1);
                }
                Ordering::Equal if settings.unique => {
                    eprintln!(
                        "sort: {}:{}: disorder: {}",
                        settings.checked_file_name.as_ref().unwrap(),
                        idx + 1,
                        line
                    );
                    std::process::exit(1);
                }
                _ => (),
            };
        }
        prev = Some(line.clone());
    }
}

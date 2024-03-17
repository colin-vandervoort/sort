use clap::{command, Arg, ArgAction};
use std::{cell, iter, path::PathBuf};

const FLAG_CHECK: &str = "check";
const FLAG_REVERSE: &str = "reverse";
const FLAG_UNIQUE: &str = "unique";
const FLAG_ZERO_TERMINATED: &str = "zero-terminated";
const UNNAMED_ARGS: &str = "file";

const STDIN_KEYWORD: &str = "-";

pub struct Settings {
    pub ascend: bool,
    pub checked_file_name: Option<String>,
    pub unique: bool,
    pub zero_terminated: bool,
}

pub enum SortInput {
    File { path: PathBuf },
    Stdin,
}

fn path_arg_to_sort_input(path: &String) -> SortInput {
    if path == STDIN_KEYWORD {
        SortInput::Stdin
    } else {
        SortInput::File {
            path: PathBuf::from(path),
        }
    }
}

pub struct App {
    pub settings: Settings,
    pub input: Vec<SortInput>,
    pub line_accumulator: cell::RefCell<Vec<String>>,
}

impl App {
    pub fn new() -> App {
        let cli_matches = command!()
            .arg(Arg::new(FLAG_CHECK).short('c').action(ArgAction::SetTrue))
            .arg(Arg::new(FLAG_REVERSE).short('r').action(ArgAction::SetTrue))
            .arg(Arg::new(FLAG_UNIQUE).short('u').action(ArgAction::SetTrue))
            .arg(Arg::new(FLAG_ZERO_TERMINATED).short('z').action(ArgAction::SetTrue))
            .arg(Arg::new(UNNAMED_ARGS).num_args(0..))
            .get_matches();

        let unnamed_args = match cli_matches.get_many::<String>(UNNAMED_ARGS) {
            Some(unnamed_arg_matches) => unnamed_arg_matches.collect(),
            None => Vec::new(),
        };

        let checked_file_name = match cli_matches.get_flag(FLAG_CHECK) {
            true if unnamed_args.is_empty() => STDIN_KEYWORD.to_string().into(),
            true => Some(unnamed_args[0].clone()),
            false => None,
        };

        let input = match unnamed_args.as_slice() {
            [] => vec![SortInput::Stdin],
            [first] => vec![path_arg_to_sort_input(&first)],
            [first, rest @ ..] => {
                if checked_file_name.is_some() {
                    vec![path_arg_to_sort_input(&first)]
                } else {
                    iter::once(first)
                        .chain(rest.iter())
                        .map(|input| path_arg_to_sort_input(input))
                        .collect()
                }
            }
        };

        App {
            settings: Settings {
                ascend: !cli_matches.get_flag(FLAG_REVERSE),
                checked_file_name,
                unique: cli_matches.get_flag(FLAG_UNIQUE),
                zero_terminated: cli_matches.get_flag(FLAG_ZERO_TERMINATED),
            },
            input,
            line_accumulator: cell::RefCell::new(Vec::new()),
        }
    }
}

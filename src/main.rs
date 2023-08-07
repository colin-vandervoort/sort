use clap::{
    command,
    Arg,
    Command,
};
use std::{io, path};

mod parse;

fn main() {
    let cli_matches = cli().get_matches();

    let file_arg_matches = cli_matches.get_many::<String>("file");
    match file_arg_matches {
        Some(values) => {
            let input_files: Vec<_> = values.collect();
            try_sort_files(input_files);
        }
        None => try_sort_stdin(),
    }
}

fn cli() -> Command {
    command!()
        // .arg(
        //     Arg::new("check")
        //     .short('c')
        //     .action(ArgAction::SetTrue)
        // )
        .arg(Arg::new("file").num_args(0..))
}

fn try_sort_files(path_strings: Vec<&String>) {
    let paths = path_strings
        .iter()
        .map(|path_str| path::Path::new(path_str));
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
            Err(_) => {
                eprintln!("sort: unknown error");
                std::process::exit(1);
            }
            _ => {}
        }
    }
}

fn try_sort_stdin() {
    let lines = io::stdin().lines();
    let mut string_vec: Vec<String> = Vec::new();
    for line in lines {
        match line {
            Ok(line_string) => parse::tokenize_line(&mut string_vec, &line_string),
            Err(error) => {
                println!("Error: {}", error)
            }
        }
    }

    string_vec.sort();

    for string in string_vec.iter() {
        println!("{}", string);
    }
}

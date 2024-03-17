mod app;
mod parse;

use std::{
    cmp::Ordering,
    fs::{self},
    io::{self},
};

use crate::app::{App, SortInput};

fn main() {
    let app = App::new();

    accumulate_lines(&app);

    if app.settings.checked_file_name.is_some() {
        check_sorted(&app);
    } else {
        sort_all(&app);
    }
}

fn accumulate_lines(app: &App) {
    for input in &app.input {
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
                    parse::tokenize_into_lines(app, &content);
                } else {
                    eprintln!("sort: Error when reading file {:?}", path);
                    std::process::exit(1);
                }
            }
            SortInput::Stdin => {
                for input_line in io::stdin().lines() {
                    match input_line {
                        Ok(line_string) => {
                            parse::tokenize_into_lines(app, &line_string);
                        }
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

fn line_order(app: &App, first_line: &String, second_line: &String) -> Ordering {
    match first_line.cmp(second_line) {
        Ordering::Greater => {
            if app.settings.ascend {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Ordering::Equal => Ordering::Equal,
        Ordering::Less => {
            if app.settings.ascend {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

fn sort_all(app: &App) {
    let mut lines = app.line_accumulator.borrow_mut();
    lines.sort_by(|a, b| line_order(app, a, b));
    if app.settings.unique {
        lines.dedup();
    }
    let line_sep = if app.settings.nul_term { "\0" } else { "\n" };
    let _ = io::stdout().lock();
    for line in lines.iter() {
        print!("{}{}", line, line_sep);
    }
}

fn check_sorted(app: &App) {
    let mut prev: Option<String> = None;
    for (idx, line) in app.line_accumulator.borrow_mut().iter().enumerate() {
        if let Some(ref prev_line) = prev {
            match line_order(app, &prev_line, &line) {
                Ordering::Greater => {
                    eprintln!(
                        "sort: {}:{}: disorder: {}",
                        app.settings.checked_file_name.as_ref().unwrap(),
                        idx + 1,
                        line
                    );
                    std::process::exit(1);
                }
                Ordering::Equal if app.settings.unique => {
                    eprintln!(
                        "sort: {}:{}: disorder: {}",
                        app.settings.checked_file_name.as_ref().unwrap(),
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

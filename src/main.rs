use std::io::{self};

fn main() {
    let line_sep = "\n";
    let lines = io::stdin().lines();
    let mut string_vec: Vec<String> = Vec::new();
    for line in lines {
        match line {
            Ok(line_string) => {
                string_vec.extend(line_string.split(line_sep).map(|token| String::from(token)));
            },
            Err(error) => println!("Error: {}", error),
        }
    }

    string_vec.sort();

    for string in string_vec.iter() {
        println!("Token: {}", string);
    }
}

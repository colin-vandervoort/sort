pub fn tokenize_line(string_vec: &mut Vec<String>, line: &str) {
    let line_sep = "\n";
    string_vec.extend(line.split(line_sep).map(|token| String::from(token)))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tokenize_line() {

//     }
// }

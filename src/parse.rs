pub fn tokenize_line<'a>(line: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
    let line_sep = "\n";
    Box::new(line.split(line_sep).map(|token| String::from(token)))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tokenize_line() {

//     }
// }
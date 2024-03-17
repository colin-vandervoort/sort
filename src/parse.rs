use crate::app::App;


pub fn tokenize_into_lines(app: &App, input: &str) {
    let sep = if app.settings.nul_term { "\0" } else { "\n" };
    app.line_accumulator
        .borrow_mut()
        .extend(input.split(sep).map(|token| String::from(token)))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tokenize_into_lines() {

//     }
// }

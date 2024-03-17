use crate::app::App;

pub fn tokenize_into_lines(app: &App, input: &str) {
    let line_sep = if app.settings.zero_terminated { "\0" } else { "\n" };
    app.line_accumulator
        .borrow_mut()
        .extend(input.split(line_sep).map(|token| String::from(token)))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_tokenize_into_lines() {

//     }
// }

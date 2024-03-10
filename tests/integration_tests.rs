use std::collections::HashMap;

mod util;

#[test]
fn test_sort_single_file() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: None,
        args: &["tests/data/breakfast.txt"],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_sort_stdin() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: Some("bananas\napples\n"),
        args: &[],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_check_file() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: None,
        args: &["-c", "tests/data/lunch.txt"],
    };
    util::cmp_actual_expect(test_params);
}

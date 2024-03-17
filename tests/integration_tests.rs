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

#[test]
fn test_check_file_unique() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: None,
        args: &["-c", "-u", "tests/data/lunch-sorted-non-uniq.txt"],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_sort_file_unique() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: None,
        args: &["-u", "tests/data/lunch-sorted-non-uniq.txt"],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_sort_numeric_en() {
    let test_params = util::TestParams {
        env_vars: HashMap::from([
            ("LANG", "en_EN")
        ]),
        stdin: None,
        args: &["-n", "tests/data/numeric.txt"],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_sort_numeric_de() {
    let test_params = util::TestParams {
        env_vars: HashMap::from([
            ("LANG", "De_DE")
        ]),
        stdin: None,
        args: &["-n", "tests/data/numeric.txt"],
    };
    util::cmp_actual_expect(test_params);
}

#[test]
fn test_zero_terminated() {
    let test_params = util::TestParams {
        env_vars: HashMap::new(),
        stdin: None,
        args: &["-z", "tests/data/nul-dip"],
    };
    util::cmp_actual_expect(test_params);
}
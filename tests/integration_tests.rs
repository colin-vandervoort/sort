use std::process::Command;

#[test]
fn test_sort_single_file() {
    let mut cmd_expect: std::process::Command = Command::new("sort");
    cmd_expect.stdin(std::process::Stdio::piped());

    let mut cmd_actual: std::process::Command = Command::new("cargo");
    cmd_actual
        .arg("run")
        .arg("--")
        .stdin(std::process::Stdio::piped());

    let common_args = vec!["tests/data/breakfast.txt"];
    for arg in common_args {
        cmd_expect.arg(arg);
        cmd_actual.arg(arg);
    }

    println!("cmd_golden: {:?}", cmd_expect);
    println!("cmd_under_test: {:?}", cmd_actual);

    let output_expect = cmd_expect
        .output()
        .expect("failed to execute process for golden command");
    let output_actual = cmd_actual
        .output()
        .expect("failed to execute process for command under test");

    assert_eq!(output_actual.status.code().unwrap(), output_expect.status.code().unwrap());
}

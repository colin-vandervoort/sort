use std::{io::Write, process::Command, process::Stdio};

mod util;

#[test]
fn test_sort_single_file() {
    let mut cmd_expect: std::process::Command = Command::new("sort");
    cmd_expect.stdin(Stdio::piped());

    let mut cmd_actual: std::process::Command = Command::new("cargo");
    cmd_actual
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .stdin(Stdio::piped());

    let common_args = vec!["tests/data/breakfast.txt"];
    cmd_expect.args(common_args.clone());
    cmd_actual.args(common_args.clone());

    println!("cmd_golden: {:?}", cmd_expect);
    println!("cmd_under_test: {:?}", cmd_actual);

    let output_expect = cmd_expect
        .output()
        .expect("failed to execute process for golden command");
    let output_actual = cmd_actual
        .output()
        .expect("failed to execute process for command under test");

    util::cmp_cmd_output(output_expect, output_actual);
}

#[test]
fn test_sort_stdin() {
    let mut cmd_expect: std::process::Command = Command::new("sort");
    cmd_expect
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut cmd_actual: std::process::Command = Command::new("cargo");
    cmd_actual
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let common_args = vec!["-"];
    cmd_expect.args(common_args.clone());
    cmd_actual.args(common_args.clone());

    println!("cmd_golden: {:?}", cmd_expect);
    println!("cmd_under_test: {:?}", cmd_actual);

    let mut child_expect = cmd_expect
        .spawn()
        .expect("failed to execute process for golden command");
    let mut child_actual = cmd_actual
        .spawn()
        .expect("failed to execute process for command under test");

    let mut stdin_expect = child_expect.stdin.take().expect("failed to take stdin");
    let handle_expect = std::thread::spawn(move || {
        stdin_expect
            .write_all("bananas\napples\n".as_bytes())
            .expect("failed to write to stdin");
    });
    let mut stdin_actual = child_actual.stdin.take().expect("failed to take stdin");
    let handle_actual = std::thread::spawn(move || {
        stdin_actual
            .write_all("bananas\napples\n".as_bytes())
            .expect("failed to write to stdin");
    });

    handle_expect.join().expect("failed to join thread");
    handle_actual.join().expect("failed to join thread");

    let output_expect = child_expect
        .wait_with_output()
        .expect("failed to read stdout");
    let output_actual = child_actual
        .wait_with_output()
        .expect("failed to read stdout");

    util::cmp_cmd_output(output_expect, output_actual);
}

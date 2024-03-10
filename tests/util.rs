use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Output, Stdio};

pub struct TestParams<'a> {
    pub env_vars: HashMap<String, String>,
    pub stdin: Option<&'static str>,
    pub args: &'a [&'a str],
}

struct CommandWrapper<'a> {
    alias: &'a str,
    cmd: Command,
}

pub fn cmp_actual_expect(params: TestParams) {
    let mut actual = CommandWrapper {
        alias: "actual",
        cmd: Command::new("cargo"),
    };
    actual.cmd.arg("run").arg("--quiet").arg("--");

    let mut expect = CommandWrapper {
        alias: "expect",
        cmd: Command::new("sort"),
    };

    let [child_actual, child_expect] = [&mut actual, &mut expect].map(|cmd_wrapper| {
        cmd_wrapper.cmd.args(params.args);
        cmd_wrapper.cmd.envs(params.env_vars.clone());
        cmd_wrapper
            .cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        println!("cmd_{}: {:?}", cmd_wrapper.alias, cmd_wrapper.cmd);

        let mut child = cmd_wrapper
            .cmd
            .spawn()
            .expect(format!("failed to execute process for cmd_{0}", cmd_wrapper.alias).as_str());

        let mut child_stdin = child.stdin.take().expect("failed to take stdin");
        let handle = std::thread::spawn(move || {
            if let Some(stdin_str) = params.stdin.clone() {
                child_stdin
                    .write_all(stdin_str.as_bytes())
                    .expect("failed to write to stdin");
            }
        });
        handle
            .join()
            .expect(format!("failed to join thread for cmd_{0}", cmd_wrapper.alias).as_str());

        child
    });

    let output_actual = child_actual
        .wait_with_output()
        .expect("failed to read stdout");
    let output_expect = child_expect
        .wait_with_output()
        .expect("failed to read stdout");

    cmp_cmd_output(output_expect, output_actual);
}

pub fn cmp_cmd_output(output_expect: Output, output_actual: Output) {
    let expect_stdout_string = String::from_utf8(output_expect.stdout).expect("not valid utf8");
    let actual_stdout_string = String::from_utf8(output_actual.stdout).expect("not valid utf8");
    println!("expect-stdout: {}", expect_stdout_string);
    println!("actual-stdout: {}", actual_stdout_string);
    assert_eq!(actual_stdout_string, expect_stdout_string);

    let expect_stderr_string = String::from_utf8(output_expect.stderr).expect("not valid utf8");
    let actual_stderr_string = String::from_utf8(output_actual.stderr).expect("not valid utf8");
    println!("expect-stderr: {}", expect_stderr_string);
    println!("actual-stderr: {}", actual_stderr_string);
    assert_eq!(actual_stderr_string, expect_stderr_string);

    println!("expect-status: {}", output_expect.status);
    println!("actual-status: {}", output_actual.status);
    assert_eq!(output_expect.status, output_actual.status);
}

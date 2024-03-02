// #[macro_export]
// macro_rules! cmp_cmd {
//     ($test_name:ident, $cmd_golden:expr, $cmd_under_test:expr) => {
//         #[test]
//         fn $test_name() {
//             let mut cmd_golden: std::process::Command = $cmd_golden;
//             let mut cmd_under_test: std::process::Command = $cmd_under_test;

//             let output_expect = cmd_golden
//                 .output()
//                 .expect("failed to execute process for golden command");
//             let output_actual = cmd_under_test
//                 .output()
//                 .expect("failed to execute process for command under test");

//             assert_eq!(output_actual)
//         }
//     };
// }

use std::process::Output;

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

use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn integration_test1() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    /*
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("infalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
    */
}


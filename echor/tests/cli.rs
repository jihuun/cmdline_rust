use anyhow::Result; // anyhow: https://sftblw.tistory.com/106
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    //let mut cmd = Command::cargo_bin("echor").unwrap();
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure();  // check the command exit with non-0
    //cmd.assert().success();  // check the command exit with 0
    cmd.assert()
        .stderr(predicate::str::contains("Usage")); // check err massage has Usage string.
                                        
    // check the echor "hello" command's exit status is not zero
    cmd.arg("hello").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt");
    Ok(())
}

#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt");
    Ok(())
}
#[test]
fn hello3() -> Result<()> {
    run(&["Hello there", "-n"], "tests/expected/hello3.txt");
    Ok(())
}
#[test]
fn hello4() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello4.txt");
    Ok(())
}

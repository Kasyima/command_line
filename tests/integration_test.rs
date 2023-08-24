use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("command_line")
        .expect("binary exists")
        .assert()
        // The assert() produces an Assert struct. The Assert struct gives you various utility functions
        // for assertions of the status and output of the executed command.
        .success()
        .stdout(predicate::str::contains("Meow!"));
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("command_line")
        .expect("binary exists")
        .args(&["-f", "no/such?file.txt"])
        .assert()
        .failure();

    Ok(())
}

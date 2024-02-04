#[cfg(test)]
extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use std::process::Command;

// CLI Test
//
// Test the CLI interface of the application. This is done by running the binary
// and expecting that the execution is passed to Git.
//
// In this case, we expect the initial output to be the usage of Git.
#[test]
fn test_cli() {
    let expected_initial = "usage: git [-v | --version]";

    let mut cmd = Command::cargo_bin("giton").expect("Calling binary failed");

    cmd.assert()
        .stdout(predicate::str::contains(expected_initial));
}

// CLI Version Test
//
// This is a similar test to the previous one, but we are testing the ability to
// pass the `version` argument to the binary.
#[test]
fn test_version() {
    let expected_initial = "git version";

    let mut cmd = Command::cargo_bin("giton").expect("Calling binary failed");

    cmd.arg("version")
        .assert()
        .stdout(predicate::str::contains(expected_initial));
}

// CLI Passthrough Test
//
// This tests the ability to call Clap commands. The command tested here is
// `onconfig`.
#[test]
fn test_passthrough() {
    let expected_initial = "+--";

    let mut cmd = Command::cargo_bin("giton").expect("Calling binary failed");

    cmd.arg("onconfig")
        .assert()
        .stdout(predicate::str::contains(expected_initial));
}

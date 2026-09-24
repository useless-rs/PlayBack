//! End-to-end checks for the installable CLI.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_exits_successfully() {
    let mut command = Command::cargo_bin("playback-cli").expect("binary should build");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

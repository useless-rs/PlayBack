//! End-to-end checks for the root installable binary.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn root_binary_help_exits_successfully() {
    let mut command = Command::cargo_bin("playback").expect("root binary should build");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn empty_command_prints_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hclrs")?;
    cmd.assert()
       .stdout(predicate::str::contains("Usage: "));

    Ok(())
}

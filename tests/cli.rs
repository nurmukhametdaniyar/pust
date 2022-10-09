use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

// TODO: Fix tests by adding entering master password

#[test]
fn run_usually() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pust")?;

    cmd.arg("test");
    cmd.assert().success();

    Ok(())
}

#[test]
fn run_with_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pust")?;
    let password_length = 16;
    cmd.arg("test")
        .arg("--length")
        .arg(password_length.to_string());
    cmd.assert().success();
    let output = cmd.output().unwrap();
    let password = str::from_utf8(&output.stdout).unwrap().trim().to_string();
    assert_eq!(password.len(), password_length);
    Ok(())
}

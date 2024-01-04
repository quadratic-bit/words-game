use assert_cmd::Command;
use assert_fs::{NamedTempFile, assert::PathAssert};
use predicates::prelude::*;
use predicate::str::contains as str_contains;

use word_chain_game::literals::*;

#[test]
fn invalid_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("words")?;

    cmd.arg("--foo").arg("bar");
    cmd.assert()
        .failure()
        .stderr(str_contains(ERR_INVALID_CLI_ARGUMENT));

    Ok(())
}

#[test]
fn help_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("words")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(str_contains(HELP_STRING));

    Ok(())
}

#[test]
fn save_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new("result.txt")?;
    let mut cmd = Command::cargo_bin("words")?;

    cmd.arg("--save")
        .arg(file.path())
        .write_stdin(String::from("fine\n") + COMMAND_EXIT);
    cmd.assert()
        .success()
        .stdout(str_contains(INF_WELCOME).and(str_contains(INF_EXIT)));

    file.assert("fine");
    file.close()?;

    Ok(())
}

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/// Compare contents of test file with `stdout`
fn run(args: &[&str], file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(file_name)?;
    Command::cargo_bin("rust-cat")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rust-cat")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

#[test]
fn file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rust-cat")?
        .arg("somewhere/does/exist")
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn one_file_without_arg() -> Result<(), Box<dyn std::error::Error>> {
    run(&["Cargo.toml"], "tests/expected/one_file_without_arg.txt")
}

#[test]
fn two_file_without_arg() -> Result<(), Box<dyn std::error::Error>> {
    run(
        &["Cargo.toml", "Cargo.toml"],
        "tests/expected/two_file_without_arg.txt",
    )
}

#[test]
fn one_file_with_arg_n() -> Result<(), Box<dyn std::error::Error>> {
    run(
        &["-n", "Cargo.toml"],
        "tests/expected/one_file_with_arg_n.txt",
    )
}

#[test]
fn one_file_with_arg_b() -> Result<(), Box<dyn std::error::Error>> {
    run(
        &["-b", "Cargo.toml"],
        "tests/expected/one_file_with_arg_b.txt",
    )
}

#[test]
fn two_file_with_arg_n() -> Result<(), Box<dyn std::error::Error>> {
    run(
        &["-n", "Cargo.toml", "Cargo.toml"],
        "tests/expected/two_file_with_arg_n.txt",
    )
}

#[test]
fn two_file_with_arg_b() -> Result<(), Box<dyn std::error::Error>> {
    run(
        &["-b", "Cargo.toml", "Cargo.toml"],
        "tests/expected/two_file_with_arg_b.txt",
    )
}

#[test]
fn one_file_with_arg_n_and_b() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rust-cat")?
        .arg("-n")
        .arg("-b")
        .arg("Cargo.toml")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: The argument '--number-nonblank' cannot be used with '--number'",
        ));
    Ok(())
}

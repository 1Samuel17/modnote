// use predicates::prelude::*;
// use assert_cmd::cargo::cargo_bin;

// #[test]
// fn test_default_greeting() {
//     let mut cmd = assert_cmd::Command::cargo_bin("rust-cli-template").unwrap();
//     cmd.assert().success().stdout(predicate::str::contains("Hello, World!"));
// }

// #[test]
// fn test_custom_name() {
//     let mut cmd = assert_cmd::Command::cargo_bin("rust-cli-template").unwrap();
//     cmd.arg("--name")
//         .arg("Rust")
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("Hello, Rust!"));
// }

// #[test]
// fn test_verbose_flag() {
//     let mut cmd = assert_cmd::Command::cargo_bin("rust-cli-template").unwrap();
//     cmd.arg("--verbose")
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("Running in verbose mode"));
// }

// #[test]
// fn test_help_flag() {
//     let mut cmd = assert_cmd::Command::cargo_bin("rust-cli-template").unwrap();
//     cmd.arg("--help")
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("A template for Rust CLI applications"));
// }

// #[test]
// fn test_version_flag() {
//     let mut cmd = assert_cmd::Command::cargo_bin("rust-cli-template").unwrap();
//     cmd.arg("--version").assert().success().stdout(predicate::str::contains("rust-cli-template"));
// }

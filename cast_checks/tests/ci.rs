use assert_cmd::Command;
use regex::Regex;
use std::{
    env::{remove_var, set_current_dir},
    path::Path,
};
use tempfile::tempdir;

#[ctor::ctor]
fn initialize() {
    remove_var("CARGO_TERM_COLOR");
    set_current_dir("..").unwrap();
}

#[test]
fn clippy() {
    Command::new("cargo")
        .args([
            "clippy",
            "--all-features",
            "--all-targets",
            "--",
            "--deny=warnings",
            "--warn=clippy::pedantic",
            "--allow=clippy::let-underscore-untyped",
            "--allow=clippy::missing-panics-doc",
        ])
        .assert()
        .success();
}

#[test]
fn license() {
    let re = Regex::new(r"^[^:]*\b(Apache-2.0|BSD-3-Clause|MIT)\b").unwrap();

    for line in std::str::from_utf8(
        &Command::new("cargo")
            .arg("license")
            .assert()
            .get_output()
            .stdout,
    )
    .unwrap()
    .lines()
    {
        assert!(re.is_match(line), "{line:?} does not match");
    }
}

#[test]
fn markdown_link_check() {
    let tempdir = tempdir().unwrap();

    Command::new("npm")
        .args(["install", "markdown-link-check"])
        .current_dir(&tempdir)
        .assert()
        .success();

    let readme_md = Path::new(env!("CARGO_MANIFEST_DIR")).join("../README.md");

    Command::new("npx")
        .args(["markdown-link-check", &readme_md.to_string_lossy()])
        .current_dir(&tempdir)
        .assert()
        .success();
}

#[test]
fn prettier() {
    let tempdir = tempdir().unwrap();

    Command::new("npm")
        .args(["install", "prettier"])
        .current_dir(&tempdir)
        .assert()
        .success();

    Command::new("npx")
        .args([
            "prettier",
            "--check",
            &format!("{}/../**/*.md", env!("CARGO_MANIFEST_DIR")),
            &format!("{}/../**/*.yml", env!("CARGO_MANIFEST_DIR")),
            &format!("!{}/../target/**", env!("CARGO_MANIFEST_DIR")),
        ])
        .current_dir(&tempdir)
        .assert()
        .success();
}

#[test]
fn sort() {
    Command::new("cargo")
        .args(["sort", "--check"])
        .assert()
        .success();
}

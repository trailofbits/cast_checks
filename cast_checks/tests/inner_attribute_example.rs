use assert_cmd::Command;
use std::env::remove_var;
use tempfile::tempdir;

#[ctor::ctor]
fn initialize() {
    remove_var("CARGO_TERM_COLOR");
}

#[test]
fn build() {
    run_command("build", |mut command| {
        command.env("CAST_CHECKS_LOG", "1");
        command.assert().success().stdout(
            "\
cast_checks rewriting `x as u16` at src/lib.rs:3:0
cast_checks not descending into `mod c;` at src/lib.rs:3:0
",
        );
    });
}

#[test]
fn test() {
    run_command("test", |mut command| {
        command.assert().failure().stdout(
            predicates::str::is_match(
                r"\
thread 'checked_truncation' \([0-9]*\) panicked at src/lib\.rs:3:1:
invalid cast in `x as u16` at src/lib\.rs:3:0: TryFromIntError\(\(\)\)
",
            )
            .unwrap(),
        );
    });
}

fn run_command(subcommand: &str, f: fn(Command)) {
    let tempdir = tempdir().unwrap();

    let mut command = Command::new("cargo");
    command
        .args([
            subcommand,
            "--target-dir",
            &tempdir.path().to_string_lossy(),
        ])
        .current_dir("../inner_attribute_example")
        .env_remove("RUSTUP_TOOLCHAIN");

    f(command);
}

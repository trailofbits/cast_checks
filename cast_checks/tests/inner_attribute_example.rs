use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn build() {
    run_command("build", |mut command| {
        command.env("CAST_CHECKS_LOG", "1");
        command.assert().success().stdout(predicates::str::contains(
            "cast_checks rewriting `x as u16` at src/lib.rs:3:1",
        ));
    });
}

#[test]
fn test() {
    run_command("test", |mut command| {
        command.assert().failure().stdout(predicates::str::contains(
            "thread 'checked_truncation' panicked at 'invalid cast in `x as u16` at src/lib.rs:3:1: TryFromIntError(())', src/lib.rs:3:1"
        ));
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

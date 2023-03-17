use assert_cmd::Command;
use tempfile::tempdir;

const LOCATIONS: &[&str] = &[
    "cast_checks/tests/basic.rs:22:13",
    "cast_checks/tests/basic.rs:30:13",
    "cast_checks/tests/basic.rs:40:13",
    "cast_checks/tests/basic.rs:49:13",
];

#[test]
fn accuracy() {
    let tempdir = tempdir().unwrap();

    let output = Command::new("cargo")
        .args([
            "test",
            "--test=basic",
            "--features=__no_should_panic",
            "--target-dir",
            &tempdir.path().to_string_lossy(),
            "--",
            "--test-threads=1",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let locations = std::str::from_utf8(&output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.starts_with("thread") {
                line.split(' ').last()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(LOCATIONS, locations);
}

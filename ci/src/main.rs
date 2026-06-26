use anyhow::{Result, anyhow, bail};
use cargo_metadata::{Message, camino::Utf8PathBuf};
use std::{path::Path, process::Command};

fn main() {
    let executable = test_executable().unwrap();
    let status = Command::new(executable)
        .current_dir(workspace_root())
        .status()
        .unwrap();
    assert!(status.success());
}

fn test_executable() -> Result<Utf8PathBuf> {
    let mut command = Command::new("cargo");
    let output = command
        .args([
            "build",
            "--manifest-path",
            "ci/Cargo.toml",
            "--tests",
            "--message-format=json",
        ])
        .current_dir(workspace_root())
        .output()?;
    if !output.status.success() {
        bail!("command failed: {command:?}");
    }
    let messages =
        Message::parse_stream(output.stdout.as_slice()).collect::<Result<Vec<_>, _>>()?;
    let executables = messages
        .into_iter()
        .filter_map(|message| {
            let Message::CompilerArtifact(artifact) = message else {
                return None;
            };
            if artifact.target.name != "ci" || !artifact.target.is_bin() || !artifact.profile.test {
                return None;
            }
            artifact.executable
        })
        .collect::<Vec<_>>();
    if executables.len() >= 2 {
        bail!("found multiple test executables: {executables:?}");
    }
    executables
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("found no test executables"))
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("ci crate should be under workspace root")
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use regex::Regex;
    use std::path::Path;
    use tempfile::tempdir;

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
    fn format() {
        Command::new("rustup")
            .args(["run", "nightly", "cargo", "fmt", "--check"])
            .assert()
            .success();
    }

    #[test]
    fn license() {
        let re = Regex::new(r"^[^:]*\b(Apache-2.0|BSD-3-Clause|MIT|Zlib)\b").unwrap();

        for line in std::str::from_utf8(
            &Command::new("cargo")
                .arg("license")
                .env_remove("CARGO_TERM_COLOR")
                .assert()
                .success()
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
}

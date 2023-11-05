use rustc_version::{version_meta, Channel};

fn main() {
    if enabled("CAST_CHECKS_LOG") {
        if matches!(version_meta().unwrap().channel, Channel::Nightly) {
            println!("cargo:rustc-cfg=procmacro2_semver_exempt");
        } else {
            println!(
                "cargo:warning=`CAST_CHECKS_LOG` is enabled, but this option requires a nightly \
                 compiler"
            );
        }
    }

    println!("cargo:rerun-if-env-changed=CAST_CHECKS_LOG");
}

fn enabled(key: &str) -> bool {
    std::env::var(key).map_or(false, |value| value != "0")
}

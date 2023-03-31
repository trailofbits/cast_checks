fn main() {
    if enabled("CAST_CHECKS_LOG") && cfg!(not(procmacro2_semver_exempt)) {
        println!(
            "cargo:warning=`CAST_CHECKS_LOG` is enabled, but this option requires \
             `--cfg procmacro2_semver_exempt` to be passed to rustc"
        );
    }

    println!("cargo:rerun-if-env-changed=CAST_CHECKS_LOG");
}

fn enabled(key: &str) -> bool {
    std::env::var(key).map_or(false, |value| value != "0")
}

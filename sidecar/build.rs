use std::env;

// runs at compile time, and produces environment variables that we tell cargo to pass to rustc.
// the end result is that the output env variables are packaged into the binary and can be read at runtime
// by the application.
fn main() {
    let version: String = env::var("SIDECAR_VERSION").unwrap_or(String::from("development"));
    let git_commit: String = env::var("SIDECAR_GIT_COMMIT").unwrap_or_default();
    println!("cargo:rustc-env=SIDECAR_VERSION={}", version);
    println!("cargo:rustc-env=SIDECAR_GIT_COMMIT={}", git_commit);
}

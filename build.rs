use std::env;
use std::process::Command;

fn main() {
    // Get the current build profile (debug or release)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Check if the build profile is debug
    if profile == "debug" {
        // Only run npm build during debug mode
        println!("cargo:warning=Running 'npm run build' in debug mode...");

        // Run 'npm run build' for frontend assets
        let status = Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir("client") // Make sure to adjust this path if necessary
            .status()
            .expect("Failed to run 'npm run build'");

        if !status.success() {
            panic!(
                "'npm run build' failed with exit code: {}",
                status.code().unwrap_or(-1)
            );
        }
    } else {
        println!("cargo:warning=Skipping 'npm run build' in release mode...");
    }
}

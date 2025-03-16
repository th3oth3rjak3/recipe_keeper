use std::{env, process::Command};

fn main() {
    // Get the current build profile (e.g., "debug" or "release")
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Only run the build logic in release mode
    if profile == "release" {
        println!("Running build.rs in release mode");
        // Add your build logic here, e.g., compiling assets, linking, etc.
        let client_dir = "client"; // Directory containing package.json

        // Run `vite build`
        let status = Command::new("npm")
            .args(["run", "build"])
            .current_dir(client_dir)
            .status()
            .expect("Failed to run npm run build");

        if !status.success() {
            panic!("npm run build failed");
        }
    } else {
        println!("Skipping build.rs in debug mode");
    }
}

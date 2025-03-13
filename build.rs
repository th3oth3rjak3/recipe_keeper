use std::process::Command;

fn main() {
    // Run the npm build command
    println!("Running 'npm run build'...");

    let status = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("client") // Specify the directory where your client (frontend) code is located
        .status()
        .expect("Failed to run 'npm run build'");

    if !status.success() {
        panic!("npm build failed!");
    }

    // Optional: tell Cargo to rerun the build script if any files in the client directory change
    println!("cargo:rerun-if-changed=client/");
}

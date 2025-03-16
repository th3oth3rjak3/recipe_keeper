use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let client_dir = "client"; // Directory containing package.json
    let output_dir = "static"; // Where the build output should go

    // Ensure `npm install` is run first (optional but recommended)
    let status = Command::new("npm")
        .arg("install")
        .current_dir(client_dir)
        .status()
        .expect("Failed to run npm install");

    if !status.success() {
        panic!("npm install failed");
    }

    // Run `vite build`
    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(client_dir)
        .status()
        .expect("Failed to run npm run build");

    if !status.success() {
        panic!("npm run build failed");
    }

    // Ensure the `static` directory exists
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir).expect("Failed to remove old static directory");
    }

    // Move the `dist` output to `static`
    fs::rename(format!("{}/dist", client_dir), output_dir)
        .expect("Failed to move built frontend to static directory");

    // Notify Cargo when to rerun this build script
    println!("cargo:rerun-if-changed={}/package.json", client_dir);
    println!("cargo:rerun-if-changed={}/vite.config.js", client_dir);
    println!("cargo:rerun-if-changed={}/src", client_dir);
}

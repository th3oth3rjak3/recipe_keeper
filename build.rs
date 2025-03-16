use std::process::Command;

fn main() {
    let client_dir = "client"; // Directory containing package.json

    // Run `vite build`
    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(client_dir)
        .status()
        .expect("Failed to run npm run build");

    if !status.success() {
        panic!("npm run build failed");
    }

    // Notify Cargo when to rerun this build script
    println!("cargo:rerun-if-changed={}/package.json", client_dir);
    println!("cargo:rerun-if-changed={}/vite.config.js", client_dir);
    println!("cargo:rerun-if-changed={}/src", client_dir);
}

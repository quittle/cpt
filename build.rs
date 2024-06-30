use std::process::Command;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=package.json");
    println!("cargo::rerun-if-changed=package-lock.json");
    println!("cargo::rerun-if-changed=src/web_actor/static");

    assert!(Command::new("npm").args(["ci"]).status().unwrap().success());
    assert!(Command::new("npx")
        .args(["npm", "run", "build"])
        .status()
        .unwrap()
        .success());
}

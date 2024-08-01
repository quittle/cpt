// This is needed for the build to provide the the OUT_DIR build env flag
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
}

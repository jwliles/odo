fn main() {
    // Only rebuild if the build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
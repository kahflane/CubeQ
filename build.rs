fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    cubeq_buildinfo::emit();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    cubeq_buildinfo::emit();
    println!("cargo:rerun-if-changed=windows/app.manifest");

    // Embed the PerMonitorV2 DPI manifest so desktop coordinates are physical
    // pixels on Windows (computer use relies on this).
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")
            .ok_or_else(|| std::io::Error::other("Cargo did not provide CARGO_MANIFEST_DIR"))?;
        let manifest = std::path::PathBuf::from(manifest_dir)
            .join("windows/app.manifest")
            .canonicalize()?;

        println!("cargo:rustc-link-arg-bin=cubeq=/MANIFEST:EMBED");
        println!(
            "cargo:rustc-link-arg-bin=cubeq=/MANIFESTINPUT:{}",
            manifest.display()
        );
        println!("cargo:rustc-link-arg-bin=cubeq=/MANIFESTUAC:NO");
    }

    Ok(())
}

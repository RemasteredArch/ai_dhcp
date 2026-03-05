use std::{env, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Put `memory.x` in our output directory and ensure it's on the linker's search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))?.write_all(include_bytes!("memory.x"))?;
    println!("cargo:rustc-link-search={}", out.display());

    // Tell `cargo` to rerun this script if `memory.x` changes.
    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}

// This module's implementation is heavily modified from
// <https://github.com/embassy-rs/embassy/blob/5c1ca25/examples/rp/build.rs>.
// Original Embassy source code copyright Dario Nieuwenhuis, licensed under the
// [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the
// [MIT License](https://opensource.org/license/MIT).

use std::{error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    register_memory_link_script()
}

fn register_memory_link_script() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Put `memory.x` in our output directory and ensure it's on the linker's search path.
    File::create(out_dir.join("memory.x"))?.write_all(include_bytes!("memory.x"))?;
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}

// build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let faucon_dir = env::var("FAUCON_DIR").unwrap();
    println!("cargo:rerun-if-changed={}/faucon.asm", faucon_dir);
    println!("cargo:rerun-if-changed={}/faucon_fw.bin", faucon_dir);

    let faucon_meta = std::fs::metadata(format!("{}/faucon_fw.bin", faucon_dir)).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("falcon_fw.rs");
    let mut f = File::create(&dest_path).unwrap();

    Command::new("make")
        .current_dir(faucon_dir)
        .output()
        .expect("failed to execute falcon fw compilation");

    f.write_all(b"use libtegra::tsec::Firmware;").unwrap();
    f.write_all(format!("static FALCON_FW: Firmware<u8, {}> = Firmware::new(*include_bytes!(concat!(env!(\"FAUCON_DIR\"),\"faucon_fw.bin\")));", faucon_meta.len()).as_bytes()).unwrap();
}

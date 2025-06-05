use std::{env::var, fs::write, path::Path, time::Instant};

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("now.txt");
    let now = format!("{:?}", Instant::now());
    assert!(!path.exists());
    write(&path, now).unwrap();
    println!("cargo::rerun-if-changed={out_dir}");
}

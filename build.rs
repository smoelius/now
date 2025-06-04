use std::{
    env::var,
    fs::{read_to_string, write},
    path::PathBuf,
    time::Instant,
};

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let path = PathBuf::from(out_dir).join("now.txt");
    let now = format!("{:?}", Instant::now());
    if path.exists() {
        let before = read_to_string(&path).unwrap();
        assert_eq!(before, now);
    }
    write(&path, now).unwrap();
    println!("cargo::rerun-if-changed={}", path.display());
}

use std::process::Command;
use std::env;
use std::path;

fn main() {
    println!("cargo:rustc-link-lib=lmdb");

    let lmdb_path = path::Path::new("deps/lmdb/libraries/liblmdb/");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = path::Path::new(&out_dir);
    let lmdb_out_path = path::Path::join(out_path, lmdb_path);

    Command::new("rm")
            .args(&["-r", lmdb_out_path.to_str().unwrap()]);
    assert!(Command::new("mkdir")
            .args(&["-p", lmdb_out_path.parent().unwrap().to_str().unwrap()])
            .status()
            .unwrap()
            .success());
    assert!(Command::new("cp")
            .args(&["-r",
                    lmdb_path.to_str().unwrap(),
                    lmdb_out_path.to_str().unwrap()])
            .status()
            .unwrap()
            .success());

    assert!(Command::new("make")
            .args(&["-C", lmdb_out_path.to_str().unwrap(),
                    &format!("-j{}", env::var("NUM_JOBS").unwrap())])
            .status()
            .unwrap()
            .success());
    println!("cargo:rustc-link-search=native={}", lmdb_out_path.to_str().unwrap());
}

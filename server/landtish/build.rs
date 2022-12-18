use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../client");

    let _child = Command::new("wasm-pack")
        .arg("build")
        .arg("../client")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("../server/web")
        .arg("--out-name")
        .arg("landtish")
        .spawn()
        .expect("failed to start wasm build");
}

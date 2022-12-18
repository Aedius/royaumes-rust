use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../client");

    let _child = Command::new("wasm-pack")
        .arg("build")
        .arg("../client")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("../server/web/description")
        .arg("--out-name")
        .arg("index")
        .spawn()
        .expect("failed to start wasm build");
}

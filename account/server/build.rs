use std::process::Command;

fn main() {

    let _child = Command::new("wasm-pack")
        .arg("build")
        .arg("../client")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("../server/web")
        .arg("--out-name")
        .arg("account")
        .spawn()
        .expect("failed to start wasm build");
}

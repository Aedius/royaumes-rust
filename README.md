# royaumes-rust
implementing an old school webgame

## requirements

frontends needs trunk :

```shell
cargo install trunk
cargo install wasm-bindgen-cli
cargo install cargo-watch
cargo install -f cargo-upgrades

rustup target add wasm32-unknown-unknown
aptitude install clang
```

## run

```shell
trunk --config public/public/Trunk.toml serve public/public/index.html

```

## build

```shell
trunk build --release public/public/index.html
```
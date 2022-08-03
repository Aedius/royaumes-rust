# royaumes-rust
implementing an old school webgame

## requirements

frontends needs trunk :

```shell
cargo install trunk
cargo install wasm-bindgen-cli
cargo install cargo-watch
cargo install -f cargo-upgrades
```

## run

```shell
trunk --config client/public/Trunk.toml serve client/public/index.html

```

## build

```shell
trunk build --release client/public/index.html
```
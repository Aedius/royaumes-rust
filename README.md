# royaumes-rust
implementing an old school webgame

## requirements

frontends needs trunk :

```shell
cargo install trunk
cargo install wasm-bindgen-cli
```

## run

```shell
trunk --config client/public-cl/Trunk.toml serve client/public-cl/index.html

```

## build

```shell
trunk build --release client/public-cl/index.html
```
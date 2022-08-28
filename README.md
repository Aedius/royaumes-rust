# royaumes-rust
implementing an old school webgame

## requirements

frontends needs trunk :

```shell
cargo install trunk
cargo install wasm-bindgen-cli
cargo install sqlx-cli
cargo install cargo-watch
cargo install -f cargo-upgrades
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
rustup target add wasm32-unknown-unknown
aptitude install clang
```

## run

see the [makefile](makefile)

## tests :

they are done with : https://cucumber-rs.github.io/cucumber/current/quickstart.html
```
cargo test -p account-model --test account-model
```

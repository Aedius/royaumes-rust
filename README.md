# royaumes-rust
implementing an old school webgame

## lib:

an [auth](lib/auth/README.md) lib to share the jwt token check to all component.

## components :

a [public](public/README.md) component to handle anonymous traffic, present the game.

an [account](account/README.md) component with : 
- webcomponent to register / login

a [server](server/README.md) component with :
- tbd

a [planet](planet/README.md) component with :
- tbd

an [army](army/README.md) component with :
- tbd

## Development

### requirements

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

### run

see the [makefile](makefile)

### tests :

they are done with : https://cucumber-rs.github.io/cucumber/current/quickstart.html
```
cargo test -p account-state --test account-state
```

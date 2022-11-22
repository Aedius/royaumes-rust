# royaumes-rust
implementing an old school webgame

## lib:

an [state](lib/state/README.md) main lib about event sourcing, it provide a trait to implement to create a state from event and generate event from command.

an [state-repository](lib/state-repository/README.md) lib that handle the evenstore database with the event command and state provided by the previous lib.

an [auth](lib/auth/README.md) lib to share the jwt token check to all component.

an [global-config](lib/global-config/README.md) lib to share the configuration between services for CORS



## components :

a [public](public/README.md) html / js client to load public resources.

a [private](private/README.md) html / js client to load private resources.

an [account](account/README.md) component with : 
- webcomponent to register / login



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

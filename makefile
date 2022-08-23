public-serve:
	trunk --config public/Trunk.toml serve public/index.html

account-client:
	cargo watch -w account/client -- trunk --config account/client/Trunk.toml build account/client/index.html

account-client-component:
	wasm-pack build account/client-component --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -i account/server/web -- cargo run --color=always -p account-server

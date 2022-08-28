public-serve:
	trunk --config public/Trunk.toml serve public/index.html

account-client:
	cargo watch -w account/client -w account/api -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/api -w account/model -i account/server/web -- cargo run --color=always -p account-server

public-serve:
	trunk --config public/Trunk.toml serve public/index.html


## 	account

account-client:
	cargo watch -w account/client -w account/api -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/api -w account/model -i account/server/web -- cargo run --color=always -p account-server


## server

server-client:
	cargo watch -w server/client -w server/api -- wasm-pack build server/client --target web --out-dir ../server/web --out-name server

server-server:
	cargo watch -w server/server -w server/api -w server/model -i server/server/web -- cargo run --color=always -p server-server


## sqlx

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


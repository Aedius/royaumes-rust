public-serve:
	trunk --config public/Trunk.toml serve public/index.html


## 	account

account-client:
	cargo watch -w account/client -w account/api -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/api -w account/model -i account/server/web -- cargo run --color=always -p account-server


## game

game-client:
	cargo watch -w game/client -w game/api -- wasm-pack build game/client --target web --out-dir ../server/web --out-name game

game-server:
	cargo watch -w game/server -w game/api -w game/model -i game/server/web -- cargo run --color=always -p game-server


## sqlx

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


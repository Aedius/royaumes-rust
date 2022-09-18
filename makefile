public-serve:
	trunk --config public/Trunk.toml serve public/index.html


## 	account

account-client:
	cargo watch -w account/client -w account/api -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/api -w account/model -i account/server/web -- cargo run --color=always -p account-server


## hero

hero-client:
	cargo watch -w hero/client -w hero/api -- wasm-pack build hero/client --target web --out-dir ../server/web --out-name hero

hero-server:
	cargo watch -w hero/server -w hero/api -w hero/model -i hero/server/web -- cargo run --color=always -p hero-server


## sqlx

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


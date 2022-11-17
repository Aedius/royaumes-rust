public-serve:
	trunk --config public/Trunk.toml serve public/index.html

private-serve:
	trunk --config private/Trunk.toml serve private/index.html


## 	account

account-client:
	cargo watch -w account/client -w account/shared -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/shared -w account/state -i account/server/web -- cargo run --color=always -p account-server


## sqlx
create:
	sqlx database create --database-url mysql://root:password@localhost:3306/account

migrate:
	sqlx migrate run --source account/server/migrations --database-url mysql://root:password@localhost:3306/account

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


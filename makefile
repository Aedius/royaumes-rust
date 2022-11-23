public-serve:
	trunk --config public/Trunk.toml serve public/index.html

## 	account

account-client:
	cargo watch -w account/client -w account/shared -- wasm-pack build account/client --target web --out-dir ../server/web --out-name account

account-server:
	cargo watch -w account/server -w account/shared -w account/state -i account/server/web -- cargo run --color=always -p account-server

### server Landtish

landtish-client:
	cargo watch -w server/landtish/client -w server/landtish/shared -- wasm-pack build server/landtish/client --target web --out-dir ../server/web/description --out-name index

landtish-server:
	cargo watch -w server/landtish/server -w server/landtish/shared -w server/landtish/state -i server/landtish/server/web -- cargo run --color=always -p landtish-server




## sqlx
create:
	sqlx database create --database-url mysql://root:password@localhost:3306/account

migrate:
	sqlx migrate run --source account/server/migrations --database-url mysql://root:password@localhost:3306/account

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


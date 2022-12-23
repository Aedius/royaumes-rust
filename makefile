public-serve:
	trunk --config public/Trunk.toml serve public/index.html

### account

account-server:
	cargo run --color=always -p account-server

### bank

bank-server:
	cargo run --color=always -p bank-server

### server Landtish

landtish-server:
	cargo run --color=always -p landtish-server


## sqlx
create:
	sqlx database create --database-url mysql://root:password@localhost:3306/account

migrate:
	sqlx migrate run --source account/server/migrations --database-url mysql://root:password@localhost:3306/account

generate-sqlx-data:
	sqlx prepare --merged --database-url mysql://root:password@localhost:3306/account


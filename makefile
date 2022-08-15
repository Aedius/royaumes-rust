web-home:
	trunk --config client/web-home/Trunk.toml serve client/web-home/index.html

srv-account:
	cargo watch -i "client/*"  -- cargo run --color=always -p account

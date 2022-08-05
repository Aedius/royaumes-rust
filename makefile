game-exe:
	cargo run --features bevy/dynamic --bin game

game-web:
	trunk --config client/game/Trunk.toml serve client/game/index.html

web-home:
	trunk --config .trunk/web-home.toml watch client/web-home/index.html

srv-account:
	cargo watch -i "client/*"  -- cargo run --color=always -p account

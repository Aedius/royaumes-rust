game-exe:
	cargo run --features bevy/dynamic --bin game

game-web:
	trunk --config client/game/Trunk.toml serve client/game/index.html

home:
	trunk --config .trunk/web-home.toml serve client/web-home/index.html
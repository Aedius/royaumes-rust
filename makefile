game-exe:
	cd client/game && cargo run --features bevy/dynamic

game-web:
	trunk --config client/game/Trunk.toml serve client/game/index.html

public-web:
	trunk --config client/public/Trunk.toml serve client/public/index.html
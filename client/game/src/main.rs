use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("1D2951").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, _: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("e3a71a").unwrap(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Up);
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut sun, mut transform) in sprite_position.iter_mut() {
        match *sun {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *sun = Direction::Down;
        } else if transform.translation.y < -200. {
            *sun = Direction::Up;
        }
    }
}
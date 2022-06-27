use bevy::prelude::*;

use std::collections::HashMap;


const GRAVITY_CONST: f32 = 0.0005;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("1D2951").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
struct Speed {
    x : f32,
    y: f32,
    w : f32,
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
            transform: Transform::from_xyz(100., -100., 0.),
            ..default()
        })
        .insert(Speed{
            x: 25.,
            y: 30.,
            w: 5000000.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("e3a71a").unwrap(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(-100., 100., 0.),
            ..default()
        })
        .insert(Speed{
            x: -8.,
            y: -10.,
            w: 20000000.,
        });
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(Entity, &mut Speed, &mut Transform)>) {

    let mut forces = HashMap::new();

    for (e, speed, item) in sprite_position.iter(){
        for (e2, speed2, item2) in sprite_position.iter(){
            if e != e2{

                let direction = (item2.translation.x - item.translation.x, item2.translation.y - item.translation.y);
                let distance = ((item2.translation.x - item.translation.x).powi(2) + (item2.translation.y - item.translation.y).powi(2)).sqrt();

                let unit_direction = (direction.0 / distance, direction.1 / distance);
                let force_scalar = GRAVITY_CONST * speed.w * speed2.w / distance.powi(2);
                let acc_scalar = force_scalar / speed.w;
                let acc_vector = (unit_direction.0 * acc_scalar, unit_direction.1 * acc_scalar);

                let acc = forces.entry(e).or_insert((0.,0.));
                *acc = (acc.0 + acc_vector.0, acc.1+ acc_vector.1);
            }
        }
    }

    for (e, mut speed, mut item) in sprite_position.iter_mut() {
        item.translation.x += speed.x  * time.delta_seconds();
        item.translation.y += speed.y * time.delta_seconds();

        let acc = forces.entry(e).or_insert((0.,0.));
        speed.x += acc.0;
        speed.y += acc.1;
    }
}


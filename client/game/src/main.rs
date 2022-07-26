use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

const GRAVITY_CONST: f32 = 0.0005;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("1D2951").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
struct AngularSpeed {
    r: f32,
    delta: f32,
    speed: f32,
    weight: f32,
}

#[derive(Component)]
struct Speed {
    x: f32,
    y: f32,
    w: f32,
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
            ..default()
        })
        .insert(AngularSpeed {
            r: 0.,
            delta: 0.,
            speed: 0.,
            weight: 10_000_000.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("e3a71a").unwrap(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        })
        .insert(AngularSpeed {
            r: 100.,
            delta: 0.,
            speed: 0.1,
            weight: 400_000.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("e3a71a").unwrap(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        })
        .insert(AngularSpeed {
            r: 300.,
            delta: 30.,
            speed: 0.05,
            weight: 1_000_000.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("e3a71a").unwrap(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        })
        .insert(AngularSpeed {
            r: 600.,
            delta: 100.,
            speed: 0.02,
            weight: 5_000_000.,
        });

    for a in -50..50 {
        for b in -50..50 {
            let x = a as f32;
            let y = b as f32;
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::hex("e3a71a").unwrap(),
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x * 10., y * 10., 0.),
                    ..default()
                })
                .insert(Speed {
                    x: 0.,
                    y: 0.,
                    w: 1000.,
                });
        }
    }
}

fn sprite_movement(
    time: Res<Time>,
    mut planetes: Query<(Entity, &mut AngularSpeed, &mut Transform), Without<Speed>>,
    mut asteroides: Query<(Entity, &mut Speed, &mut Transform), Without<AngularSpeed>>,
) {
    for (_, mut angular, mut item) in planetes.iter_mut() {
        angular.delta += time.delta_seconds() * angular.speed;
        item.translation.x = angular.r * angular.delta.cos();
        item.translation.y = angular.r * angular.delta.sin();
    }

    for (_, mut speed, mut item) in asteroides.iter_mut() {
        let mut acc = (0., 0.);
        for (_, ang_speed, bigs) in planetes.iter() {
            let direction = (
                bigs.translation.x - item.translation.x,
                bigs.translation.y - item.translation.y,
            );
            let distance = ((bigs.translation.x - item.translation.x).powi(2)
                + (bigs.translation.y - item.translation.y).powi(2))
                .sqrt();

            let unit_direction = (direction.0 / distance, direction.1 / distance);
            let force_scalar = GRAVITY_CONST * speed.w * ang_speed.weight / distance.powi(2);
            let acc_scalar = force_scalar / speed.w;
            let acc_vector = (unit_direction.0 * acc_scalar, unit_direction.1 * acc_scalar);

            acc = (acc.0 + acc_vector.0, acc.1 + acc_vector.1);
        }

        item.translation.x += speed.x * time.delta_seconds();
        item.translation.y += speed.y * time.delta_seconds();

        speed.x += acc.0;
        speed.y += acc.1;
    }
}

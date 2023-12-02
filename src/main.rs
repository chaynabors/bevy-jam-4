pub mod cli;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use clap::Parser;
use cli::Cli;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    let Cli { server: _ } = Cli::parse();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec2::new(0.0, 1.0).extend(0.0),
                scale: Vec2::new(32.0, 32.0).extend(1.0),
                ..default()
            },
            ..default()
        },
        Player { speed: 10.0 },
    ));
}

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, With<Camera>)>,
) {
    for (mut transform, _player) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::D) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y += 1.0;
        }

        if let Some(scroll) = scroll_evr.read().last() {
            let scroll = match scroll.unit {
                MouseScrollUnit::Line => scroll.y,
                MouseScrollUnit::Pixel => scroll.y / 100.0,
            };

            transform.scale.x -= scroll * 0.1;
            transform.scale.y -= scroll * 0.1;

            transform.scale = transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        }

        let scale = transform.scale.x;
        let translation = &mut transform.translation;
        *translation +=
            time.delta_seconds() * direction.normalize_or_zero().extend(0.0) * 400.0 * scale;
    }
}

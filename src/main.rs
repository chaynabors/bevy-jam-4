mod camera;
pub mod cli;
mod net;
mod player;

use bevy::prelude::*;
use camera::spawn_camera;
use clap::Parser;
use cli::Cli;
use net::NetPlugin;
use player::{spawn_player, Player};

fn main() {
    let Cli { server: _ } = Cli::parse();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            NetPlugin {
                room: "test".into(),
                players: 2,
            },
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, player::update_player)
        .add_systems(PostUpdate, camera::update_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
    let player_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    // Camera
    spawn_camera(&mut commands);

    // Player
    spawn_player(
        Player { id: 0, speed: 3.65 },
        Transform::default(),
        commands,
        player_mesh,
        player_material,
    );
}

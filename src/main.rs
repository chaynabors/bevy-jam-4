mod camera;
pub mod cli;
pub mod input;
mod net;
mod player;
mod enemy;

use bevy::prelude::*;
use bevy_ggrs::{GgrsSchedule, ReadInputs};
use camera::spawn_camera;
use clap::Parser;
use cli::Cli;
use enemy::SpawnTimer;
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
        .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(ReadInputs, input::read_local_inputs)
        .add_systems(
            GgrsSchedule,
            (
                enemy::spawn_wave,
                player::update_player.after(enemy::spawn_wave),
                enemy::update_enemy_transforms.after(player::update_player).before(camera::update_camera),
                camera::update_camera.after(player::update_player),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut server: Res<AssetServer>
) {
    let player_mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
    let player_1_material = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        unlit: true,
        ..default()
    });
    let player_2_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        unlit: true,
        ..default()
    });

    // Camera
    spawn_camera(&mut commands);

    // Player
    spawn_player(
        Player { id: 0 },
        Transform::default(),
        &mut commands,
        player_mesh.clone(),
        player_1_material,
    );

    spawn_player(
        Player { id: 1 },
        Transform::from_xyz(0.0, 0.0, 5.0),
        &mut commands,
        player_mesh,
        player_2_material,
    );

    let enemy_mesh: Handle<Mesh> = server.load("enemy1.glb#Mesh0/Primitive0");
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::MAROON,
        unlit: true,
        ..default()
    });

    enemy::setup(commands, enemy_mesh, enemy_material)
}

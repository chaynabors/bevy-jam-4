pub mod bullet;
mod camera;
pub mod cli;
mod enemy;
pub mod input;
mod net;
mod player;

use bevy::prelude::*;
use bullet::BulletPlugin;
use camera::spawn_camera;
use clap::Parser;
use cli::Cli;
use enemy::SpawnTimer;
use input::InputPlugin;
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
            },
            BulletPlugin,
            InputPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                enemy::spawn_wave,
                camera::update_camera,
                enemy::update_enemy_transforms.before(camera::update_camera),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    // Floor
    let floor_mesh: Handle<Mesh> = server.load("floor.glb#Mesh0/Primitive0");
    let floor_material = materials.add(StandardMaterial {
        unlit: true,
        fog_enabled: true,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: floor_mesh,
        material: floor_material,
        ..Default::default()
    });

    // Camera
    spawn_camera(&mut commands);

    // Player
    let player_mesh = server.load("player.glb#Mesh0/Primitive0");
    let player_1_material = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });

    spawn_player(
        Player {},
        Transform::default(),
        &mut commands,
        player_mesh.clone(),
        player_1_material,
        None,
    );

    let enemy_mesh: Handle<Mesh> = server.load("enemy1.glb#Mesh0/Primitive0");
    let enemy_material = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });

    enemy::setup(commands, enemy_mesh, enemy_material)
}

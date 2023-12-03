mod bullet;
mod camera;
mod cli;
mod enemy;
mod input;
mod net;
mod player;
mod powerups;

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bullet::BulletPlugin;
use camera::spawn_camera;
use clap::Parser;
use cli::Cli;
use input::InputPlugin;
use net::NetPlugin;
use player::{spawn_player, Player};
use powerups::PowerupPlugin;

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
            PowerupPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
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

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Player
    let player_mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
    let player_1_material = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });

    spawn_player(
        Player::new(),
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

    enemy::setup(&mut commands, enemy_mesh, enemy_material);

    powerups::spawn_powerup(
        powerups::PowerupType::Damage,
        Transform::from_translation(Vec3::new(10.0, 0.0, 10.0)),
        &mut commands,
        &server,
    );

    powerups::spawn_powerup(
        powerups::PowerupType::Speed,
        Transform::from_translation(Vec3::new(-10.0, 0.0, 10.0)),
        &mut commands,
        &server,
    );

    powerups::spawn_powerup(
        powerups::PowerupType::Health,
        Transform::from_translation(Vec3::new(10.0, 0.0, -10.0)),
        &mut commands,
        &server,
    );
}

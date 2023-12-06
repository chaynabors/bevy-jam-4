mod bullet;
mod camera;
mod cli;
mod constants;
mod enemy;
mod line_material;
mod net;
mod player;
mod powerups;
mod ship;
mod ui;
mod util;

use std::f32::consts::PI;

use bevy::{math::vec3, pbr::CascadeShadowConfigBuilder, prelude::*};
use bullet::BulletPlugin;
use camera::PlayerCameraPlugin;
use clap::Parser;
use cli::Cli;
use enemy::EnemyPlugin;
use line_material::LineMaterial;
use net::NetPlugin;
use player::PlayerPlugin;
use powerups::PowerupPlugin;
use ship::ShipPlugin;
use ui::UiPlugin;

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
            MaterialPlugin::<LineMaterial>::default(),
            NetPlugin {
                room: "test".into(),
            },
            ShipPlugin,
            PlayerPlugin,
            PlayerCameraPlugin,
            EnemyPlugin,
            BulletPlugin,
            PowerupPlugin,
            UiPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    server: Res<AssetServer>,
) {
    // Floor
    let floor_mesh: Handle<Mesh> = server.load("floor.glb#Mesh0/Primitive0");
    let floor_material = line_materials.add(LineMaterial {});

    commands.spawn(MaterialMeshBundle {
        mesh: floor_mesh,
        material: floor_material,
        transform: Transform::from_translation(vec3(0.0, -1.0, 0.0)),
        ..Default::default()
    });

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

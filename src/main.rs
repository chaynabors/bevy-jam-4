mod bullet;
mod camera;
mod cli;
mod constants;
mod enemy;
mod materials;
mod net;
mod player;
mod powerups;
mod ship;
mod ui;
mod util;

use bevy::{math::vec3, prelude::*};
use bullet::BulletPlugin;
use camera::PlayerCameraPlugin;
use clap::Parser;
use cli::Cli;
use enemy::EnemyPlugin;
use materials::{GridMaterial, ShipMaterial, SpaceMaterial};
use net::NetPlugin;
use player::PlayerPlugin;
use powerups::PowerupPlugin;
use ship::ShipPlugin;
use ui::UiPlugin;

#[derive(Debug, Default, Resource)]
struct Materials {
    ship_material: Option<Handle<ShipMaterial>>,
    grid_material: Option<Handle<GridMaterial>>,
    space_material: Option<Handle<SpaceMaterial>>,
}

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
            MaterialPlugin::<ShipMaterial>::default(),
            MaterialPlugin::<SpaceMaterial>::default(),
            MaterialPlugin::<GridMaterial>::default(),
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
        .insert_resource(Materials::default())
        .add_systems(PreStartup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Materials>,
    mut ship_materials: ResMut<Assets<ShipMaterial>>,
    mut line_materials: ResMut<Assets<GridMaterial>>,
    mut space_materials: ResMut<Assets<SpaceMaterial>>,
    server: Res<AssetServer>,
) {
    // Ship
    let ship_material = ship_materials.add(ShipMaterial::default());
    materials.ship_material = Some(ship_material);

    // Grid
    let floor_mesh: Handle<Mesh> = server.load("floor.glb#Mesh0/Primitive0");
    let space_material = space_materials.add(SpaceMaterial::default());
    materials.space_material = Some(space_material.clone());
    commands.spawn(MaterialMeshBundle {
        mesh: floor_mesh.clone(),
        material: space_material,
        transform: Transform::from_translation(vec3(0.0, -1.0, 0.0)),
        ..Default::default()
    });

    // Space
    let grid_material = line_materials.add(GridMaterial::default());
    materials.grid_material = Some(grid_material.clone());
    commands.spawn(MaterialMeshBundle {
        mesh: floor_mesh,
        material: grid_material,
        transform: Transform::from_translation(vec3(0.0, -0.99, 0.0)),
        ..Default::default()
    });
}

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    constants::{PLAYER_ACCELERATION_RATE, PLAYER_DRAG_COEFFICIENT, PLAYER_MAX_SPEED},
    materials::{GridMaterial, ShipMaterial, SpaceMaterial},
    net::{packet::PlayerState, PlayerId, PlayerPeerId},
    ship::{Ship, ShipBundle},
    Materials,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(PreUpdate, update);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub ship: ShipBundle,
}

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub damage: f32,
    pub gun: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            damage: 1.0,
            gun: 0,
        }
    }
}

fn startup(mut commands: Commands, server: Res<AssetServer>, materials: Res<Materials>) {
    commands.spawn(PlayerBundle {
        player: Player::new(),
        ship: ShipBundle {
            ship: Ship::new(
                PLAYER_MAX_SPEED,
                PLAYER_ACCELERATION_RATE,
                PLAYER_DRAG_COEFFICIENT,
            ),
            material_mesh: MaterialMeshBundle {
                mesh: server.load("player2.glb#Mesh0/Primitive0"),
                material: materials.ship_material.clone().unwrap(),
                ..Default::default()
            },
        },
    });
}

fn update(
    keys: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut ship_materials: ResMut<Assets<ShipMaterial>>,
    mut space_materials: ResMut<Assets<SpaceMaterial>>,
    mut line_materials: ResMut<Assets<GridMaterial>>,
    mut ship: Query<(&mut Ship, &Transform), (With<Player>, Without<PlayerPeerId>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut player_state_writer: EventWriter<PlayerState>,
    player_id: Res<PlayerId>,
) {
    let window = window.single();
    let (camera, global_transform) = camera.single();
    let (mut ship, transform) = ship.single_mut();

    ship.move_dir = Vec3::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        ship.move_dir.z -= 1.0;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        ship.move_dir.z += 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        ship.move_dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        ship.move_dir.x += 1.0;
    }

    let plane_origin = Vec3::new(0.0, 0.0, 0.0);
    let plane_normal = Vec3::new(0.0, 1.0, 0.0);
    let Some(viewport_position) = window.cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(global_transform, viewport_position) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(plane_origin, plane_normal) else {
        return;
    };
    ship.look_dir = ray.get_point(distance) - transform.translation;

    if let Some(ship) = &materials.ship_material {
        ship_materials.get_mut(ship.id()).unwrap().player_position = transform.translation.xz();
    }

    if let Some(line) = &materials.grid_material {
        line_materials.get_mut(line.id()).unwrap().player_position = transform.translation.xz();
    }

    if let Some(space) = &materials.space_material {
        space_materials.get_mut(space.id()).unwrap().player_position = transform.translation.xz();
    }
}

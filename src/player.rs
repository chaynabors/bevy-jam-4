use bevy::{prelude::*, window::PrimaryWindow};

pub const MAX_PLAYER_COUNT: usize = 4;
pub const PLAYER_MAX_SPEED: f32 = 6.23;
pub const PLAYER_ACCELERATION_RATE: f32 = 64.0;

use crate::ship::{Ship, ShipBundle};

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
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            damage: 1.0,
        }
    }
}

fn startup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PlayerBundle {
        player: Player::new(),
        ship: ShipBundle {
            ship: Ship::new(PLAYER_MAX_SPEED, PLAYER_ACCELERATION_RATE),
            pbr: PbrBundle {
                mesh: server.load("ship1.glb#Mesh0/Primitive0"),
                material: materials.add(StandardMaterial {
                    unlit: true,
                    ..default()
                }),
                transform: Transform::default(),
                ..default()
            },
        },
    });
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut ship: Query<(&mut Ship, &Transform), With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
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
}

use std::time::Duration;

use bevy::{math::vec3, prelude::*};

use crate::{
    player::Player,
    ship::{Ship, ShipBundle},
};

const MAX_ENEMY_COUNT: usize = 1024;
const ARENA_SIZE: f32 = 12.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnGeneration(0))
            .insert_resource(SpawnTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .add_systems(Startup, startup)
            .add_systems(Update, (spawn_wave, update_enemy));
    }
}

#[derive(Bundle, Clone)]
struct EnemyBundle {
    enemy: Enemy,
    ship: ShipBundle,
}

#[derive(Clone, Component)]
pub struct Enemy {
    id: u32,
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct SpawnGeneration(pub usize);

pub fn startup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut id = 0;
    commands.spawn_batch(
        std::iter::repeat(EnemyBundle {
            enemy: Enemy {
                id: {
                    id += 1;
                    id
                },
            },
            ship: ShipBundle {
                ship: Ship::new(4.05, 16.0),
                pbr: PbrBundle {
                    mesh: server.load("enemy1.glb#Mesh0/Primitive0"),
                    material: materials.add(StandardMaterial {
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::default(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            },
        })
        .take(MAX_ENEMY_COUNT),
    )
}

pub fn spawn_wave(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_generation: ResMut<SpawnGeneration>,
    mut enemies: Query<(&mut Transform, &mut Visibility), With<Enemy>>,
) {
    if !spawn_timer.0.just_finished() {
        spawn_timer.0.tick(time.delta());
        return;
    }
    spawn_timer.0.reset();

    spawn_timer.0.set_duration(Duration::from_secs_f32(
        5.0 + spawn_generation.0 as f32 * 0.5,
    ));

    spawn_generation.0 += 1;

    let mut spawn_count = 5 * spawn_generation.0;
    for mut enemy in enemies.iter_mut() {
        if *enemy.1 == Visibility::Hidden {
            *enemy.1 = Visibility::Visible;
            enemy.0.translation =
                vec3(fastrand::f32() - 0.5, 0.0, fastrand::f32() - 0.5).normalize() * ARENA_SIZE;
            spawn_count -= 1;
        }

        if spawn_count == 0 {
            break;
        }
    }
}

pub fn update_enemy(
    mut players: Query<(&Transform, &mut Player), Without<Enemy>>,
    mut enemies: Query<(&mut Ship, &Transform, &Visibility), With<Enemy>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut ship, transform, visibility) in &mut enemies {
        if visibility != Visibility::Hidden {
            let mut direction = Vec3::ZERO;
            let mut distance = ARENA_SIZE * 10.0;
            for (player_transform, mut player) in players.iter_mut() {
                let enemy_to_player = player_transform.translation - transform.translation;
                let enemy_to_player_len = enemy_to_player.length();
                if enemy_to_player_len < distance {
                    direction = enemy_to_player.normalize_or_zero();
                    distance = enemy_to_player_len;
                }

                if enemy_to_player_len < 0.75 {
                    player.health -= 1.0 * dt;
                }
            }

            if distance < 0.5 {
                continue;
            }

            ship.move_dir = direction;
            ship.look_dir = ship.velocity();
        }
    }
}

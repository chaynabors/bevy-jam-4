use std::time::Duration;

use bevy::{math::{vec3, vec2}, prelude::*};

use crate::{
    constants::{CHASER_ACCELERATION_RATE, CHASER_DRAG_COEFFICIENT, CHASER_MAX_SPEED},
    net::{
        packet::{EnemyState, NetworkEvent},
        ServerState,
    },
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
            .add_systems(
                Update,
                (
                    spawn_wave,
                    update_enemy,
                    net_read,
                    net_write.after(update_enemy),
                ),
            );
    }
}

#[derive(Bundle, Clone)]
struct EnemyBundle {
    enemy: Enemy,
    ship: ShipBundle,
}

#[derive(Clone, Component)]
pub struct Enemy {
    pub id: u32,
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
    let mesh = server.load("enemy1.glb#Mesh0/Primitive0");
    let material = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });
    commands.spawn_batch((0..MAX_ENEMY_COUNT).map(move |i| EnemyBundle {
        enemy: Enemy { id: i as u32 },
        ship: ShipBundle {
            ship: Ship::new(
                CHASER_MAX_SPEED,
                CHASER_ACCELERATION_RATE,
                CHASER_DRAG_COEFFICIENT,
            ),
            pbr: PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::default(),
                visibility: Visibility::Hidden,
                ..default()
            },
        },
    }))
}

pub fn spawn_wave(
    time: Res<Time>,
    status: Res<ServerState>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_generation: ResMut<SpawnGeneration>,
    mut enemies: Query<(&mut Transform, &mut Visibility), With<Enemy>>,
) {
    if *status == ServerState::Host {
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
                enemy.0.translation = vec3(fastrand::f32() - 0.5, 0.0, fastrand::f32() - 0.5)
                    .normalize()
                    * ARENA_SIZE;
                spawn_count -= 1;
            }

            if spawn_count == 0 {
                break;
            }
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

fn net_write(
    status: Res<ServerState>,
    mut net_event_writer: EventWriter<NetworkEvent>,
    ship_query: Query<(&Ship, &Transform, &Visibility, &Enemy)>,
) {
    if *status == ServerState::Host {
        net_event_writer.send_batch(ship_query.iter().map(
            |(_ship, transform, visibility, enemy)| {
                NetworkEvent::EnemyState(EnemyState {
                    id: enemy.id as u16,
                    position: vec2(transform.translation.x, transform.translation.z),
                    visible: *visibility == Visibility::Visible,
                })
            },
        ));
    }
}

fn net_read(
    status: Res<ServerState>,
    mut net_event_reader: EventReader<EnemyState>,
    mut ship_query: Query<(&mut Transform, &mut Visibility, &Enemy)>,
) {
    if *status == ServerState::Client {
        let mut ships = ship_query.iter_mut().collect::<Vec<_>>();
        ships.sort_by_key(|(_, _, enemy)| enemy.id);

        for event in net_event_reader.read() {
            let (transform, visibility, _) = ships.get_mut(event.id as usize).unwrap();
            transform.translation = vec3(event.position.x, 0.0, event.position.y);
            **visibility = if event.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

use std::time::Duration;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{
    enemy::Enemy,
    net::{
        packet::{BulletState, NetworkEvent},
        PlayerPeerId, ServerState,
    },
    player::Player,
    powerups::{PowerupSpawnEvent, PowerupType},
};

pub const MAX_BULLET_COUNT: usize = 1024;

#[derive(Debug)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::from_seconds(0.1, TimerMode::Once)))
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    update,
                    spawn_bullets,
                    net_write.after(spawn_bullets).after(update),
                    net_read,
                ),
            );
    }
}

#[derive(Resource)]
struct BulletTimer(pub Timer);

#[derive(Component, Clone)]
pub struct Bullet {
    pub id: u32,
    pub velocity: Vec2,
    pub ttl: f32,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Bundle, Clone)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub pbr: PbrBundle,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
    let material = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        unlit: true,
        ..default()
    });

    commands.spawn_batch((0..MAX_BULLET_COUNT).map(move |i| BulletBundle {
        bullet: Bullet {
            id: i as u32,
            velocity: vec2(0.0, 0.0),
            ttl: 2.0,
            damage: 1.0,
            speed: 30.0,
        },
        pbr: PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::default().with_scale(Vec3::splat(0.1)),
            visibility: Visibility::Hidden,
            ..default()
        },
    }));
}

fn update(
    mut bullets: Query<(&mut Transform, &mut Bullet, &mut Visibility), Without<Enemy>>,
    mut enemies: Query<(&Transform, &Enemy, &mut Visibility), Without<Bullet>>,
    mut spawn_powerup_events: EventWriter<PowerupSpawnEvent>,
    time: Res<Time>,
) {
    for (mut transform, bullet, _) in bullets.iter_mut() {
        transform.translation +=
            vec3(bullet.velocity.x, 0.0, bullet.velocity.y) * bullet.speed * time.delta_seconds();
    }

    for (transform, _enemy, mut visibility) in enemies.iter_mut() {
        if *visibility == Visibility::Hidden {
            continue;
        }
        for (bullet_transform, _bullet, mut bullet_vis) in bullets.iter_mut() {
            if *bullet_vis == Visibility::Visible {
                if (transform.translation.xz() - bullet_transform.translation.xz()).length() < 0.5 {
                    *bullet_vis = Visibility::Hidden;
                    *visibility = Visibility::Hidden;

                    // 5% chance to spawn a powerup
                    if fastrand::f32() < 0.05 {
                        spawn_powerup_events.send(PowerupSpawnEvent {
                            powerup_type: PowerupType::random(),
                            transform: transform.clone(),
                        })
                    }
                }
            }
        }
    }

    for (_, mut bullet, mut visibility) in bullets.iter_mut() {
        bullet.ttl -= time.delta_seconds();
        if bullet.ttl < 0.0 {
            *visibility = Visibility::Hidden;
        }
    }
}

fn spawn_bullets(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut timer: ResMut<BulletTimer>,
    mut player: Query<(&mut Player, &Transform), Without<PlayerPeerId>>,
    mut bullets: Query<
        (&mut Transform, &mut Bullet, &mut Visibility),
        (Without<Enemy>, Without<Player>),
    >,
) {
    let (mut player, transform) = player.single_mut();

    timer.0.tick(time.delta());
    if keys.any_pressed([KeyCode::Space]) && timer.0.finished() {
        let spread = 0.10;

        player.gun = (player.gun + 1) % 2;

        let mut position = transform.translation.xz();
        let mut forward = transform.forward() * 0.3;
        position += forward.xz();
        forward.y = 0.0;
        forward = forward.normalize_or_zero();

        let mut side = transform.right() * 0.65;
        if player.gun != 0 {
            side *= -1.0;
        }
        position += side.xz();

        let velocity = vec2(
            forward.x + fastrand::f32() * spread - spread / 2.0,
            forward.z + fastrand::f32() * spread - spread / 2.0,
        );

        for (mut transform, mut bullet, mut visibility) in bullets.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                transform.translation = vec3(position.x, 0.5, position.y);
                bullet.velocity = velocity;
                bullet.ttl = 2.0;
                break;
            }
        }

        timer
            .0
            .set_duration(Duration::from_secs_f32(0.1 / player.damage));

        timer.0.reset();
    }
}

fn net_write(
    status: Res<ServerState>,
    bullet_query: Query<(&Transform, &Visibility, &Bullet)>,
    mut net_event_writer: EventWriter<NetworkEvent>,
) {
    if *status == ServerState::Host {
        net_event_writer.send_batch(bullet_query.iter().map(|(transform, visibility, bullet)| {
            NetworkEvent::BulletState(BulletState {
                id: bullet.id,
                position: vec2(transform.translation.x, transform.translation.z),
                velocity: bullet.velocity,
                visible: *visibility == Visibility::Visible,
            })
        }));
    }
}

fn net_read(
    status: Res<ServerState>,
    mut net_event_reader: EventReader<BulletState>,
    mut bullet_query: Query<(&mut Transform, &mut Visibility, &mut Bullet)>,
) {
    if *status == ServerState::Client {
        let mut ships = bullet_query.iter_mut().collect::<Vec<_>>();
        ships.sort_by_key(|(_, _, enemy)| enemy.id);

        for event in net_event_reader.read() {
            let (transform, visibility, bullet) = ships.get_mut(event.id as usize).unwrap();
            transform.translation = vec3(event.position.x, 0.0, event.position.y);
            **visibility = if event.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            bullet.velocity = event.velocity;
            bullet.ttl = 2.0;
        }
    }
}

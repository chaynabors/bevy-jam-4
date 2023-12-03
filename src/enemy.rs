use bevy::{prelude::*, math::vec3};

use crate::player::Player;

const ENEMY_SPEED: f32 = 4.2;
const ARENA_SIZE: f32 = 10.0;

#[derive(Bundle, Clone)]
struct EnemyBundle {
    enemy: Enemy,
    pbr: PbrBundle,
}

#[derive(Clone, Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn setup(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>) {
        commands.spawn_batch(std::iter::repeat(EnemyBundle {
            enemy: Enemy,
            pbr: PbrBundle {
                mesh,
                material,
                transform: Transform::default(),
                visibility: Visibility::Hidden,
                ..default()
            },
        }).take(1000))
}

pub fn spawn_wave(
    mut enemies: Query<(&mut Transform, &mut Visibility), With<Enemy>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if !spawn_timer.0.just_finished() {
        spawn_timer.0.tick(time.delta());
        return;
    }
    spawn_timer.0.reset();

    let mut spawn_count = 10;
    for mut enemy in enemies.iter_mut() {
        if *enemy.1 == Visibility::Hidden {
            *enemy.1 = Visibility::Visible;
            enemy.0.translation = vec3(fastrand::f32() - 0.5, 0.0, fastrand::f32() - 0.5).normalize() * ARENA_SIZE;
            spawn_count -= 1;
        }

        if spawn_count == 0 {
            break;
        }
    }
}

pub fn update_enemy_transforms(
    players: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for mut enemy in &mut enemies {
        let mut direction = Vec3::ZERO;
        let mut distance = ARENA_SIZE * 10.0;
        for player in &players {
            let enemy_to_player = player.translation - enemy.translation;
            let enemy_to_player_len = enemy_to_player.length();
            if enemy_to_player_len < distance {
                direction = enemy_to_player.normalize_or_zero();
                distance = enemy_to_player_len;
            }
        }

        if distance < 0.5 {
            continue;
        }

        enemy.translation += direction * ENEMY_SPEED * dt;
        enemy.look_to(direction, Vec3::Y);
    }
}

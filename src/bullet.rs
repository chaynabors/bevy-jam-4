use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::{enemy::Enemy, player::Player};

const MAX_BULLET_COUNT: usize = 1024 * 10;

#[derive(Component, Clone)]
pub struct Bullet {
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

#[derive(Debug)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update);
    }
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

    commands.spawn_batch(
        std::iter::repeat(BulletBundle {
            bullet: Bullet {
                velocity: vec2(0.0, 0.0),
                ttl: 2.0,
                damage: 1.0,
                speed: 30.0,
            },
            pbr: PbrBundle {
                mesh,
                material,
                transform: Transform::default().with_scale(Vec3::splat(0.1)),
                visibility: Visibility::Hidden,
                ..default()
            },
        })
        .take(MAX_BULLET_COUNT),
    );
}

fn update(
    mut bullets: Query<(&mut Transform, &mut Bullet, &mut Visibility), Without<Enemy>>,
    mut enemies: Query<(&Transform, &Enemy, &mut Visibility), Without<Bullet>>,
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

pub fn spawn_bullet(
    bullets: &mut Query<(&mut Transform, &mut Bullet, &mut Visibility), (Without<Enemy>, Without<Player>)>,
    position: Vec2,
    velocity: Vec2,
) {
    for (mut transform, mut bullet, mut visibility) in bullets.iter_mut() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
            transform.translation = vec3(position.x, 0.5, position.y);
            bullet.velocity = velocity;
            bullet.ttl = 2.0;
            break;
        }
    }
}

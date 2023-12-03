use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::enemy::Enemy;

#[derive(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub ttl: f32,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub pbr: PbrBundle,
}

#[derive(Debug)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet), Without<Enemy>>,
    mut enemies: Query<(Entity, &Transform, &Enemy, &mut Visibility), Without<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, bullet) in bullets.iter_mut() {
        transform.translation +=
            vec3(bullet.velocity.x, 0.0, bullet.velocity.y) * bullet.speed * time.delta_seconds();

        if transform.translation.length() > 100.0 {
            commands.entity(entity).despawn();
        }
    }

    for (_entity, transform, _enemy, mut visibility) in enemies.iter_mut() {
        for (bullet_entity, bullet_transform, _bullet) in bullets.iter_mut() {
            if *visibility == Visibility::Hidden {
                continue;
            }
            if (transform.translation.xz() - bullet_transform.translation.xz()).length() < 0.5 {
                commands.entity(bullet_entity).despawn();
                *visibility = Visibility::Hidden;
            }
        }
    }

    for (entity, _, mut bullet) in bullets.iter_mut() {
        bullet.ttl -= time.delta_seconds();
        if bullet.ttl < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec2,
    velocity: Vec2,
) {
    let mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
    let material = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        unlit: true,
        ..default()
    });

    commands.spawn(BulletBundle {
        bullet: Bullet {
            damage: 1.0,
            speed: 30.0,
            velocity,
            ttl: 2.0,
        },
        pbr: PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(vec3(position.x, 0.5, position.y))
                .with_scale(Vec3::splat(0.1)),
            ..Default::default()
        },
    });
}

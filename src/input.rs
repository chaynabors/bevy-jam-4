use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::camera::Camera,
    window::PrimaryWindow,
};

use crate::{
    bullet::spawn_bullet,
    net::packet::NetEvent,
    player::{NetPlayer, Player, PLAYER_SPEED},
};

#[derive(Resource)]
struct BulletTimer(pub Timer);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::from_seconds(0.00001, TimerMode::Once)))
            .add_systems(PreUpdate, (read_input, read_bullet_input.after(read_input)));
    }
}

fn read_input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Player, &mut Transform), Without<NetPlayer>>,
    mut tx_net_event: EventWriter<NetEvent>,
) {
    let (_player, mut transform) = player.single_mut();

    let dt = time.delta_seconds();
    let mut direction = Vec2::ZERO;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y += 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        transform.translation += vec3(direction.x, 0.0, direction.y) * PLAYER_SPEED * dt;
        transform.look_to(vec3(direction.x, 0.0, direction.y), Vec3::Y);
        tx_net_event.send(NetEvent::PlayerUpdate(transform.translation));
    }
}

fn read_bullet_input(
    mut command: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    player: Query<(&Player, &Transform), Without<NetPlayer>>,
    mut tx_net_event: EventWriter<NetEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut timer: ResMut<BulletTimer>,
) {
    let window = window.single();
    let (camera, global_transform) = camera.single();
    let (_player, transform) = player.single();

    timer.0.tick(time.delta());
    if keys.any_pressed([KeyCode::Space]) && timer.0.finished() {
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
        let global_cursor = ray.get_point(distance);

        let direction = (global_cursor - transform.translation).normalize();

        let spread = 0.50;

        let position = vec2(transform.translation.x, transform.translation.z);

        let velocity = vec2(
            direction.x + fastrand::f32() * spread - spread / 2.0,
            direction.z + fastrand::f32() * spread - spread / 2.0,
        );

        spawn_bullet(
            &mut command,
            &mut meshes,
            &mut materials,
            position,
            velocity,
        );

        tx_net_event.send(NetEvent::NewBullet { position, velocity });

        timer.0.reset();
    }
}

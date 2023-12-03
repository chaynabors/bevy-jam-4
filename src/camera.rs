use bevy::{math::vec3, prelude::*};

use crate::player::NetPlayer;
use crate::player::Player;

pub fn spawn_camera(commands: &mut Commands) -> Entity {
    commands.spawn(Camera3dBundle::default()).id()
}

pub fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<NetPlayer>, Without<Camera>)>,
) {
    let transform = players.single();

    *camera.single_mut() =
        Transform::from_translation(transform.translation + vec3(0.0, 1.0, 1.0).normalize() * 20.0)
            .looking_at(transform.translation, Vec3::Y);
}

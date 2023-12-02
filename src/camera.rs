use bevy::{math::vec3, prelude::*};

use crate::player::Player;

pub fn spawn_camera(commands: &mut Commands) -> Entity {
    commands.spawn(Camera3dBundle::default()).id()
}

pub fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    players: Query<(&Player, &Transform), Without<Camera>>,
) {
    if let Some(player) = players.iter().find(|p| p.0.id == 0) {
        *camera.single_mut() = Transform::from_translation(
            player.1.translation + vec3(0.0, 1.0, 1.0).normalize() * 10.0,
        )
        .looking_at(player.1.translation, Vec3::Y);
    }
}

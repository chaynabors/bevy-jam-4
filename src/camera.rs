use bevy::{math::vec3, prelude::*};
use bevy_ggrs::LocalPlayers;

use crate::{player::Player, enemy::Enemy};

pub fn spawn_camera(commands: &mut Commands) -> Entity {
    commands.spawn(Camera3dBundle::default()).id()
}

pub fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    players: Query<(&Player, &Transform), (Without<Camera>, Without<Enemy>)>,
    local_players: Res<LocalPlayers>,
) {
    let Some(local_player) = local_players.0.first() else {
        return;
    };

    if let Some((_, transform)) = players.iter().find(|(p, _)| p.id == *local_player) {
        *camera.single_mut() =
        Transform::from_translation(transform.translation + vec3(0.0, 1.0, 1.0).normalize() * 10.0)
            .looking_at(transform.translation, Vec3::Y);
    };
}
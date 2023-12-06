use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};

use crate::{net::PlayerPeerId, player::Player};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PostUpdate, update_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        tonemapping: Tonemapping::None,
        ..default()
    });
}

fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>, Without<PlayerPeerId>)>,
) {
    let transform = player.single();

    *camera.single_mut() = Transform::from_translation(
        transform.translation + Vec3::new(0.0, 1.0, 0.5).normalize() * 40.0,
    )
    .looking_at(transform.translation, Vec3::NEG_Z);
}

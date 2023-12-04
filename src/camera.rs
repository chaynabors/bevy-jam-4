use bevy::{math::vec3, prelude::*};

use crate::player::Player;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PostUpdate, update_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let transform = player.single();

    *camera.single_mut() =
        Transform::from_translation(transform.translation + vec3(0.0, 1.5, 1.0).normalize() * 20.0)
            .looking_at(transform.translation, Vec3::Y);
}

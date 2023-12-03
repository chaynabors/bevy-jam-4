use bevy::{
    math::vec3,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs};

use crate::{
    input::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP},
    net::Config,
};

const PLAYER_SPEED: f32 = 6.1;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    pbr: PbrBundle,
    not_shadow_caster: NotShadowCaster,
    not_shadow_receiver: NotShadowReceiver,
}

#[derive(Component)]
pub struct Player {
    pub id: usize,
}

pub fn spawn_player(
    player: Player,
    transform: Transform,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> Entity {
    commands
        .spawn(PlayerBundle {
            player,
            pbr: PbrBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            not_shadow_caster: NotShadowCaster,
            not_shadow_receiver: NotShadowReceiver,
        })
        .add_rollback()
        .id()
}

pub fn update_player(
    mut player: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    inputs: Res<PlayerInputs<Config>>,
) {
    for (player, mut transform) in player.iter_mut() {
        let (input, _) = inputs[player.id];
        let dt = time.delta_seconds();

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y += 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }

        direction = direction.clamp_length_max(1.0);

        transform.translation += vec3(direction.x, 0.0, direction.y) * PLAYER_SPEED * dt;
    }
}
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_matchbox::matchbox_socket::PeerId;

pub const PLAYER_SPEED: f32 = 6.1;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    pbr: PbrBundle,
    not_shadow_caster: NotShadowCaster,
    not_shadow_receiver: NotShadowReceiver,
}

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            speed: PLAYER_SPEED,
            damage: 1.0,
        }
    }
}

#[derive(Component)]
pub struct NetPlayer(pub PeerId);

pub fn spawn_player(
    player: Player,
    transform: Transform,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    peer_id: Option<PeerId>,
) -> Entity {
    let mut entity = commands.spawn(PlayerBundle {
        player,
        pbr: PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        not_shadow_caster: NotShadowCaster,
        not_shadow_receiver: NotShadowReceiver,
    });

    if let Some(peer_id) = peer_id {
        entity.insert(NetPlayer(peer_id));
    }

    entity.id()
}

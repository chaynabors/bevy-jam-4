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
pub struct Player {}

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

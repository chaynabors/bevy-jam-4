use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_matchbox::matchbox_socket::PeerId;

use crate::{input::InputState, net::packet::NetEvent};

pub const PLAYER_SPEED: f32 = 6.1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update_player_transform);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub pbr: PbrBundle,
    pub not_shadow_caster: NotShadowCaster,
    pub not_shadow_receiver: NotShadowReceiver,
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

fn startup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PlayerBundle {
        player: Player::new(),
        pbr: PbrBundle {
            mesh: server.load("ship1.glb#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                unlit: true,
                ..default()
            }),
            transform: Transform::default(),
            ..default()
        },
        not_shadow_caster: NotShadowCaster,
        not_shadow_receiver: NotShadowReceiver,
    });
}

fn update_player_transform(
    time: Res<Time>,
    input: Res<InputState>,
    mut player: Query<(&Player, &mut Transform), Without<NetPlayer>>,
    mut tx_net_event: EventWriter<NetEvent>,
) {
    let (player, mut transform) = player.single_mut();

    let dt = time.delta_seconds();

    if (input.planar_cursor_position - transform.translation).length() > 0.1 {
        transform.look_at(input.planar_cursor_position, Vec3::Y);
    }

    if input.move_dir.length_squared() > 0.1 {
        transform.translation += input.move_dir * player.speed * dt;
    }

    tx_net_event.send(NetEvent::PlayerState {
        position: transform.translation,
        rotation: transform.rotation,
    });
}

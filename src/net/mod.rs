pub mod packet;

use bevy::{
    math::vec3,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_matchbox::{
    matchbox_socket::{MultipleChannels, PeerId, PeerState, WebRtcSocketBuilder},
    MatchboxSocket,
};

use crate::{
    bullet::Bullet,
    player::{PLAYER_ACCELERATION_RATE, PLAYER_MAX_SPEED},
    ship::{Ship, ShipBundle},
};

use self::packet::NetEvent;

#[derive(Debug, Clone, Resource)]
pub struct PlayerId(pub usize);

#[derive(Component)]
pub struct PlayerPeerId(pub PeerId);

#[derive(Debug, Clone, Resource)]
pub struct NetData {
    room: String,
}

#[derive(Debug, Clone)]
pub struct NetPlugin {
    pub room: String,
}

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetData {
            room: self.room.clone(),
        })
        .add_event::<NetEvent>()
        .add_systems(Startup, startup)
        .add_systems(Update, update);
    }
}

fn startup(mut commands: Commands, net_data: Res<NetData>) {
    let room_url = format!("wss://bevy-jam-4.fly.dev/{}", net_data.room);
    info!(%room_url, "connecting to matchbox server");
    commands.insert_resource(MatchboxSocket::from(
        WebRtcSocketBuilder::new(room_url)
            .add_unreliable_channel()
            .add_reliable_channel(),
    ));
}

fn update(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    mut tx_net_event: EventReader<NetEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut peer_ships: Query<(Entity, &Ship, &mut Transform, &PlayerPeerId)>,
    mut bullets: Query<(&mut Transform, &mut Bullet, &mut Visibility), Without<Ship>>,
) {
    let peer_updates = socket.update_peers();
    for (peer_id, peer_state) in peer_updates {
        match peer_state {
            PeerState::Connected => {
                info!(%peer_id, "connected to peer");

                commands.spawn((
                    ShipBundle {
                        ship: Ship::new(PLAYER_MAX_SPEED, PLAYER_ACCELERATION_RATE),
                        pbr: PbrBundle {
                            mesh: server.load("ship1.glb#Mesh0/Primitive0"),
                            material: materials.add(StandardMaterial {
                                unlit: true,
                                ..default()
                            }),
                            transform: Transform::default(),
                            ..default()
                        },
                    },
                    PlayerPeerId(peer_id),
                ));
            }
            PeerState::Disconnected => {
                info!(%peer_id, "disconnected from peer");

                // find player entity and despawn
                if let Some((entity, _, _, _)) = peer_ships
                    .iter_mut()
                    .find(|(_, _, _, PlayerPeerId(id))| id == &peer_id)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    let peers = socket.connected_peers().collect::<Vec<_>>();

    let events = tx_net_event.read().collect::<Vec<_>>();
    if !events.is_empty() {
        for peer_id in &peers {
            for event in &events {
                socket
                    .get_channel(0)
                    .unwrap()
                    .send(packet::to_net_packet(event), peer_id.clone());
            }
        }
    }

    for (peer_id, data) in socket.get_channel(0).unwrap().receive() {
        if let Some(event) = packet::from_net_packet::<NetEvent>(&data) {
            match event {
                NetEvent::PlayerState { position, rotation } => {
                    info!(%peer_id, ?position, "received player update");
                    if let Some((_, _, mut transform, _)) = peer_ships
                        .iter_mut()
                        .find(|(_, _, _, PlayerPeerId(id))| id == &peer_id)
                    {
                        transform.translation = position;
                        transform.rotation = rotation;
                    }
                }
                NetEvent::NewBullet { position, velocity } => {
                    for (mut transform, mut bullet, mut visibility) in bullets.iter_mut() {
                        if *visibility == Visibility::Hidden {
                            *visibility = Visibility::Visible;
                            transform.translation = vec3(position.x, 0.5, position.y);
                            bullet.velocity = velocity;
                            bullet.ttl = 2.0;
                            break;
                        }
                    }
                }
            }
        }
    }
}

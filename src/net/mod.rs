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

use self::packet::{ClientEvent, ServerEvent};

#[derive(Debug, Clone, Eq, PartialEq, Resource)]
pub enum ServerState {
    Host,
    Client,
    Unknown,
}

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
        .insert_resource(ServerState::Unknown)
        .add_event::<ClientEvent>()
        .add_event::<ServerEvent>()
        .add_systems(Startup, startup)
        .add_systems(Update, update);
    }
}

fn startup(mut commands: Commands, net_data: Res<NetData>) {
    let room_url = "ws://[::]:3536";
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
    mut state: ResMut<ServerState>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut peer_ships: Query<(Entity, &Ship, &mut Transform, &PlayerPeerId)>,
    mut bullets: Query<(&mut Transform, &mut Bullet, &mut Visibility), Without<Ship>>,

    mut read_client_events: EventReader<ClientEvent>,
    mut read_server_events: EventReader<ServerEvent>,
) {
    let peer_updates = socket.update_peers();

    if *state == ServerState::Unknown {
        if socket.connected_peers().count() > 0 {
            *state = ServerState::Host;
        } else {
            *state = ServerState::Client;
        }
    }

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

    for (peer_id, data) in socket.get_channel(0).unwrap().receive() {
        if let Some(net_packet) = packet::from_net_packet(&data) {
            match net_packet {
                packet::NetPacket::Client(events) => {
                    // client should not receive client events
                    if *state == ServerState::Client {
                        continue;
                    }

                    for event in events {
                        match event {
                            ClientEvent::PlayerState { position, rotation } => todo!(),
                            ClientEvent::BulletState { position, velocity } => todo!(),
                        }
                    }
                }
                packet::NetPacket::Server(events) => {
                    // server should not receive server events
                    if *state == ServerState::Host {
                        continue;
                    }

                    for event in events {
                        match event {
                            ServerEvent::PlayerState {
                                peer_id,
                                position,
                                rotation,
                            } => {
                                info!(%peer_id, ?position, "received player update");
                                if let Some((_, _, mut transform, _)) = peer_ships
                                    .iter_mut()
                                    .find(|(_, _, _, PlayerPeerId(id))| id == &peer_id)
                                {
                                    transform.translation = vec3(position.x, 0.0, position.y);
                                    transform.rotation = Quat::from_rotation_y(rotation);
                                }
                            }
                            ServerEvent::BulletState {
                                id: _,
                                position,
                                velocity,
                            } => {
                                for (mut transform, mut bullet, mut visibility) in
                                    bullets.iter_mut()
                                {
                                    if *visibility == Visibility::Hidden {
                                        *visibility = Visibility::Visible;
                                        transform.translation = vec3(position.x, 0.5, position.y);
                                        bullet.velocity = velocity;
                                        bullet.ttl = 2.0;
                                        break;
                                    }
                                }
                            }
                            ServerEvent::EnemyState {
                                id,
                                position,
                                velocity,
                            } => todo!(),
                        }
                    }
                }
            }
        }
    }

    if *state == ServerState::Host {
        let events = read_server_events.read().cloned().collect::<Vec<_>>();
        if !events.is_empty() {
            let net_packet = packet::to_net_packet(&packet::NetPacket::Server(events));
            for peer_id in &peers {
                socket
                    .get_channel(0)
                    .unwrap()
                    .send(net_packet.clone(), peer_id.clone());
            }
        }
    } else if *state == ServerState::Client {
        let events = read_client_events.read().cloned().collect::<Vec<_>>();
        if !events.is_empty() {
            let net_packet = packet::to_net_packet(&packet::NetPacket::Client(events));
            for peer_id in &peers {
                socket
                    .get_channel(0)
                    .unwrap()
                    .send(net_packet.clone(), peer_id.clone());
            }
        }
    }
}

pub mod packet;

use bevy::{math::vec3, prelude::*};
use bevy_matchbox::{
    matchbox_socket::{MultipleChannels, PeerId, PeerState, WebRtcSocketBuilder},
    MatchboxSocket,
};

use crate::{
    bullet::Bullet,
    constants::{PLAYER_ACCELERATION_RATE, PLAYER_DRAG_COEFFICIENT, PLAYER_MAX_SPEED},
    enemy::Enemy,
    ship::{Ship, ShipBundle},
};

use self::packet::{BulletState, Connected, Disconnected, EnemyState, NetworkEvent, PlayerState};

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
        .add_event::<NetworkEvent>()
        .add_event::<Connected>()
        .add_event::<Disconnected>()
        .add_event::<PlayerState>()
        .add_event::<EnemyState>()
        .add_event::<BulletState>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                read_events,
                connected_handler,
                disconnected_handler,
                player_state_handler,
                enemy_state_handler,
                bullet_state_handler,
            ),
        );
    }
}

fn startup(mut commands: Commands, net_data: Res<NetData>) {
    let room_url = "wss://bevy-jam-4.fly.dev";
    info!(%room_url, "connecting to matchbox server");

    commands.insert_resource(MatchboxSocket::from(
        WebRtcSocketBuilder::new(room_url)
            .add_unreliable_channel()
            .add_reliable_channel(),
    ));
}

fn read_events(
    mut state: ResMut<ServerState>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    mut read_events: EventReader<NetworkEvent>,
    mut write_connected: EventWriter<Connected>,
    mut write_disconnected: EventWriter<Disconnected>,
    mut write_player_state: EventWriter<PlayerState>,
    mut write_enemy_state: EventWriter<EnemyState>,
    mut write_bullet_state: EventWriter<BulletState>,
) {
    let peer_updates = socket.update_peers();

    if *state == ServerState::Unknown {
        if socket.connected_peers().count() > 0 {
            *state = ServerState::Host;
            info!("hosting game");
        } else {
            *state = ServerState::Client;
            info!("joining game");
        }
    }

    for (peer_id, peer_state) in peer_updates {
        match peer_state {
            PeerState::Connected => {
                info!(%peer_id, "connected to peer");
                write_connected.send(Connected { peer_id });
            }
            PeerState::Disconnected => {
                info!(%peer_id, "disconnected from peer");
                write_disconnected.send(Disconnected { peer_id });
            }
        }
    }

    for (peer_id, data) in socket.get_channel(0).unwrap().receive() {
        if let Some(net_packet) = packet::bytes_to_net_packet(&data) {
            for packet in net_packet.0 {
                match packet {
                    NetworkEvent::PlayerState(state) => write_player_state.send(state),
                    NetworkEvent::EnemyState(state) => write_enemy_state.send(state),
                    NetworkEvent::BulletState(state) => write_bullet_state.send(state),
                }
            }
        }
    }

    let events = read_events.read().cloned().collect::<Vec<_>>();
    if !events.is_empty() {
        let net_packet = packet::net_packet_to_bytes(&packet::NetPacket(events));
        let peers = socket.connected_peers().collect::<Vec<_>>();
        for peer_id in peers {
            socket
                .get_channel(0)
                .unwrap()
                .send(net_packet.clone(), peer_id.clone());
        }
    }
}

fn connected_handler(
    mut commands: Commands,
    mut reader: EventReader<Connected>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    for event in reader.read() {
        commands.spawn((
            ShipBundle {
                ship: Ship::new(
                    PLAYER_MAX_SPEED,
                    PLAYER_ACCELERATION_RATE,
                    PLAYER_DRAG_COEFFICIENT,
                ),
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
            PlayerPeerId(event.peer_id),
        ));
    }
}

fn disconnected_handler(
    mut commands: Commands,
    mut reader: EventReader<Disconnected>,
    mut peer_ships: Query<(Entity, &Ship, &mut Transform, &PlayerPeerId)>,
) {
    for event in reader.read() {
        // find player entity and despawn
        if let Some((entity, _, _, _)) = peer_ships
            .iter_mut()
            .find(|(_, _, _, PlayerPeerId(id))| id == &event.peer_id)
        {
            commands.entity(entity).despawn();
        }
    }
}

fn player_state_handler(
    mut peer_ships: Query<(Entity, &Ship, &mut Transform, &PlayerPeerId)>,
    mut reader: EventReader<PlayerState>,
) {
    for event in reader.read() {
        if let Some((_, _, mut transform, _)) = peer_ships
            .iter_mut()
            .find(|(_, _, _, PlayerPeerId(id))| id == &event.id)
        {
            transform.translation = vec3(event.position.x, 0.0, event.position.y);
            transform.rotation = Quat::from_rotation_y(event.rotation);
        }
    }
}

fn bullet_state_handler(
    mut bullets: Query<(&mut Transform, &mut Bullet, &mut Visibility)>,
    mut reader: EventReader<BulletState>,
) {
    for event in reader.read() {
        for (mut transform, mut bullet, mut visibility) in bullets.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                transform.translation = vec3(event.position.x, 0.5, event.position.y);
                bullet.velocity = event.velocity;
                bullet.ttl = 2.0;
                break;
            }
        }
    }
}

fn enemy_state_handler(
    mut enemies: Query<(&mut Ship, &Transform, &Visibility), With<Enemy>>,
    mut reader: EventReader<EnemyState>,
) {
}

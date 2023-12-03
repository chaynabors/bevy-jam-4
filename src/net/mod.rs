pub mod packet;

use bevy::prelude::*;
use bevy_matchbox::{
    matchbox_socket::{MultipleChannels, PeerId, PeerState, WebRtcSocketBuilder},
    MatchboxSocket,
};

use crate::player::{spawn_player, NetPlayer, Player};

use self::packet::NetEvent;

#[derive(Debug, Clone, Resource)]
pub struct NetData {
    room: String,
    peers: Vec<PeerId>,
}

#[derive(Debug, Clone)]
pub struct NetPlugin {
    pub room: String,
}

#[derive(Debug, Clone, Resource)]
pub struct PlayerId(pub usize);

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetData {
            room: self.room.clone(),
            peers: Vec::new(),
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
            .add_reliable_channel()
            .add_unreliable_channel(),
    ));
}

fn update(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    mut net_data: ResMut<NetData>,
    mut tx_net_event: EventReader<NetEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut players: Query<(Entity, &Player, &mut Transform, &NetPlayer)>,
) {
    let peer_updates = socket.update_peers();
    for (peer_id, peer_state) in peer_updates {
        match peer_state {
            PeerState::Connected => {
                info!(%peer_id, "connected to peer");
                net_data.peers.push(peer_id);

                let player_mesh = meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap());
                let player_1_material = materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    unlit: true,
                    ..default()
                });

                spawn_player(
                    Player {},
                    Transform::default(),
                    &mut commands,
                    player_mesh,
                    player_1_material,
                    Some(peer_id),
                );
            }
            PeerState::Disconnected => {
                info!(%peer_id, "disconnected from peer");
                net_data.peers.retain(|&id| id != peer_id);

                // find player entity and despawn
                if let Some((entity, _, _, _)) = players
                    .iter_mut()
                    .find(|(_, _, _, NetPlayer(id))| id == &peer_id)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    let events = tx_net_event.read().collect::<Vec<_>>();
    if !events.is_empty() {
        for peer_id in &net_data.peers {
            for event in &events {
                socket
                    .get_channel(0)
                    .unwrap()
                    .send(packet::to_net_packet(event), *peer_id);
            }
        }
    }

    for (peer_id, data) in socket.get_channel(0).unwrap().receive() {
        let event = packet::from_net_packet::<NetEvent>(&data);

        match event {
            NetEvent::PlayerUpdate(pos) => {
                info!(%peer_id, ?pos, "received player update");
                if let Some((_, _, mut transform, _)) = players
                    .iter_mut()
                    .find(|(_, _, _, NetPlayer(id))| id == &peer_id)
                {
                    transform.translation = pos;
                }
            }
        }
    }
}

use bevy::prelude::*;
use bevy_ggrs::{
    ggrs::{self},
    GgrsApp, GgrsPlugin,
};
use bevy_matchbox::{
    matchbox_socket::{PeerId, SingleChannel},
    MatchboxSocket,
};

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Debug, Clone, Resource)]
pub struct NetData {
    room: String,
    players: u8,
}

#[derive(Debug, Clone)]
pub struct NetPlugin {
    pub room: String,
    pub players: u8,
}

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetData {
            room: self.room.clone(),
            players: self.players,
        })
        .add_plugins(GgrsPlugin::<Config>::default())
        .rollback_component_with_clone::<Transform>()
        .add_systems(Startup, start_server)
        .add_systems(Update, wait_for_players);
    }
}

fn start_server(mut commands: Commands, net_data: Res<NetData>) {
    let room_url = format!(
        "wss://bevy-jam-4.fly.dev/{}?next={}",
        net_data.room, net_data.players
    );
    info!(%room_url, "connecting to matchbox server");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
}

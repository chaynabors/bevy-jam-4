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

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<Config>::default())
            .rollback_component_with_clone::<Transform>()
            .add_systems(Startup, start_server)
            .add_systems(Update, wait_for_players);
    }
}

fn start_server(mut commands: Commands) {
    let room_url = "wss://bevy-jam-4.fly.dev/extreme_bevy?next=2";
    info!("connecting to matchbox server: {room_url}");
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

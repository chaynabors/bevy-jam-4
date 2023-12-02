mod state;
mod topology;

use std::net::{Ipv6Addr, SocketAddrV6};

use crate::{
    state::{RequestedRoom, RoomId, ServerState},
    topology::MatchmakingDemoTopology,
};
use axum::{http::StatusCode, response::IntoResponse, routing::get};
use matchbox_signaling::SignalingServerBuilder;
use tracing::info;
use tracing_subscriber::prelude::*;

fn setup_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=info,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(false)
                .with_target(false),
        )
        .init();
}

#[tokio::main]
async fn main() {
    setup_logging();

    let host = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 3536, 0, 0);
    info!(%host, "Matchbox Signaling Server");

    // Setup router
    let mut state = ServerState::default();
    let server = SignalingServerBuilder::new(host, MatchmakingDemoTopology, state.clone())
        .on_connection_request({
            let mut state = state.clone();
            move |connection| {
                let room_id = RoomId(connection.path.clone().unwrap_or_default());
                let next = connection
                    .query_params
                    .get("next")
                    .and_then(|next| next.parse::<usize>().ok());
                let room = RequestedRoom { id: room_id, next };
                state.add_waiting_client(connection.origin, room);
                Ok(true) // allow all clients
            }
        })
        .on_id_assignment({
            move |(origin, peer_id)| {
                info!("Client connected {origin:?}: {peer_id:?}");
                state.assign_id_to_waiting_client(origin, peer_id);
            }
        })
        .cors()
        .trace()
        .mutate_router(|router| {
            // Apply router transformations
            router.route("/health", get(|| async { StatusCode::OK }))
        })
        .build();
    server
        .serve()
        .await
        .expect("Unable to run signaling server, is it already running?")
}

pub async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

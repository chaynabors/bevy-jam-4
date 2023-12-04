use std::net::{Ipv6Addr, SocketAddrV6};

use axum::{http::StatusCode, response::IntoResponse, routing::get};
use matchbox_signaling::{
    topologies::client_server::{ClientServer, ClientServerState},
    SignalingServerBuilder,
};
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
    let state = ClientServerState::default();
    let server = SignalingServerBuilder::new(host, ClientServer, state.clone())
        .on_connection_request(|connection| {
            info!("Connecting: {connection:?}");
            Ok(true) // Allow all connections
        })
        .on_id_assignment(|(socket, id)| info!("{socket} received {id}"))
        .on_host_connected(|id| info!("Host joined: {id}"))
        .on_host_disconnected(|id| info!("Host left: {id}"))
        .on_client_connected(|id| info!("Client joined: {id}"))
        .on_client_disconnected(|id| info!("Client left: {id}"))
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

use bevy::prelude::*;

use crate::{
    net::{
        packet::{NetworkEvent, PlayerState},
        PlayerId, PlayerPeerId, ServerState,
    },
    player::Player,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                net_read,
                update_transforms.after(net_read),
                net_write.after(update_transforms),
            ),
        );
    }
}

#[derive(Bundle, Clone)]
pub struct ShipBundle {
    pub ship: Ship,
    pub pbr: PbrBundle,
}

#[derive(Clone, Component, Default)]
pub struct Ship {
    velocity: Vec3,
    acceleration: Vec3,
    pub move_dir: Vec3,
    pub look_dir: Vec3,
    pub max_speed: f32,
    pub acceleration_rate: f32,
    pub drag_coefficient: f32,
}

impl Ship {
    pub fn new(max_speed: f32, acceleration_rate: f32, drag_coefficient: f32) -> Self {
        Self {
            max_speed,
            acceleration_rate,
            drag_coefficient,
            ..Default::default()
        }
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
}

fn net_write(
    status: Res<ServerState>,
    mut write_player_state: EventWriter<NetworkEvent>,
    player_query: Query<(&Player, &Transform, Option<&PlayerPeerId>)>,
    player_id: Res<PlayerId>,
) {
    if *status == ServerState::Host {
        write_player_state.send_batch(player_query.iter().filter_map(
            |(player, transform, player_peer_id)| {
                Some(NetworkEvent::PlayerState(PlayerState {
                    id: player_peer_id.map(|p| p.0).unwrap_or(player_id.0?),
                    position: transform.translation,
                    rotation: transform.rotation,
                }))
            },
        ));
    }

    // only write local player state
    if *status == ServerState::Client {
        if let Some(player_peer_id) = player_id.0 {
            let player = player_query.iter().find(|(_, _, p)| p.is_none());
            if let Some((_, transform, _)) = player {
                write_player_state.send(NetworkEvent::PlayerState(PlayerState {
                    id: player_peer_id,
                    position: transform.translation,
                    rotation: transform.rotation,
                }));
            }
        }
    }
}

fn net_read(
    mut read_player_state: EventReader<PlayerState>,
    mut player_query: Query<(&mut Transform, &PlayerPeerId)>,
) {
    for player_state in read_player_state.read() {
        for (mut transform, player_peer_id) in player_query.iter_mut() {
            if player_peer_id.0 == player_state.id {
                transform.translation = player_state.position;
                transform.rotation = player_state.rotation;
            }
        }
    }
}

fn update_transforms(time: Res<Time>, mut ships: Query<(&mut Ship, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut ship, mut transform) in &mut ships {
        transform.translation =
            transform.translation + ship.velocity * dt + 0.5 * ship.acceleration * dt * dt;
        let mut new_acceleration = ship.move_dir.clamp_length_max(1.0) * ship.acceleration_rate;
        new_acceleration -= ship.velocity * ship.drag_coefficient;
        ship.velocity = (ship.velocity + 0.5 * (ship.acceleration + new_acceleration) * dt)
            .clamp_length_max(ship.max_speed);
        ship.acceleration = new_acceleration;

        let look_dir = ship.look_dir.normalize_or_zero();
        if look_dir != Vec3::ZERO {
            transform.look_to(look_dir, Vec3::Y);

            transform.rotate_axis(
                ship.acceleration.cross(Vec3::NEG_Y).normalize_or_zero(),
                ship.acceleration.length() * 0.005,
            );
        }
    }
}

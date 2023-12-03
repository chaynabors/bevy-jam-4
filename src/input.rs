use bevy::{math::vec3, prelude::*};

use crate::{
    net::packet::NetEvent,
    player::{NetPlayer, Player, PLAYER_SPEED},
};

pub fn read_input(
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Player, &mut Transform, Without<NetPlayer>)>,
    time: Res<Time>,
    mut tx_net_event: EventWriter<NetEvent>,
) {
    let (_player, mut transform, _) = player.single_mut();

    let dt = time.delta_seconds();
    let mut direction = Vec2::ZERO;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y += 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        transform.translation += vec3(direction.x, 0.0, direction.y) * PLAYER_SPEED * dt;

        tx_net_event.send(NetEvent::PlayerUpdate(transform.translation));
    }
}

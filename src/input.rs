use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::net::Config;

type InputSize = u32;

pub const INPUT_UP: InputSize = 1 << 0;
pub const INPUT_DOWN: InputSize = 1 << 1;
pub const INPUT_LEFT: InputSize = 1 << 2;
pub const INPUT_RIGHT: InputSize = 1 << 3;
pub const INPUT_FIRE: InputSize = 1 << 4;

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0;

        if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

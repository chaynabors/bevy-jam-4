use bevy::{
    math::{vec2, vec3},
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

pub fn spawn_player(
    player: Player,
    transform: Transform,
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> Entity {
    commands
        .spawn(PlayerBundle {
            player,
            pbr: PbrBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            not_shadow_caster: NotShadowCaster,
            not_shadow_receiver: NotShadowReceiver,
        })
        .id()
}

#[derive(Component)]
pub struct Player {
    pub id: u8,
    pub speed: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    pbr: PbrBundle,
    not_shadow_caster: NotShadowCaster,
    not_shadow_receiver: NotShadowReceiver,
}

pub fn update_player(
    mut player: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Some(mut player) = player.iter_mut().find(|p| p.0.id == 0) {
        let dt = time.delta_seconds();

        let dir = vec2(
            keyboard_input.pressed(KeyCode::D) as u32 as f32
                - keyboard_input.pressed(KeyCode::A) as u32 as f32,
            keyboard_input.pressed(KeyCode::W) as u32 as f32
                - keyboard_input.pressed(KeyCode::S) as u32 as f32,
        )
        .clamp_length_max(1.0);

        player.1.translation += vec3(dir.x, 0.0, dir.y) * player.0.speed * dt;
    }
}

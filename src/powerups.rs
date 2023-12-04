use bevy::prelude::*;

use crate::player::{Player, PLAYER_SPEED};

#[derive(Bundle)]
pub struct PowerupBundle {
    pub powerup: Powerup,
    pub scene: SceneBundle,
}

#[derive(Component)]
pub struct Powerup {
    pub powerup_type: PowerupType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerupType {
    Health,
    Speed,
    Damage,
}

impl PowerupType {
    pub fn random() -> Self {
        match fastrand::u32(..3) {
            0 => Self::Health,
            1 => Self::Speed,
            2 => Self::Damage,
            _ => unreachable!(),
        }
    }
}

#[derive(Event)]
pub struct PowerupSpawnEvent {
    pub powerup_type: PowerupType,
    pub transform: Transform,
}

pub struct PowerupPlugin;

impl Plugin for PowerupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PowerupSpawnEvent>()
            .add_systems(Update, update);
    }
}

fn update(
    mut commands: Commands,
    mut powerups: Query<(Entity, &mut Transform, &mut Powerup, &mut Visibility), Without<Player>>,
    mut players: Query<(Entity, &mut Transform, &mut Player), Without<Powerup>>,
    mut events: EventReader<PowerupSpawnEvent>,
    time: Res<Time>,
    server: Res<AssetServer>,
) {
    for (powerup_entity, mut powerup_transform, mut powerup, mut vis) in powerups.iter_mut() {
        powerup_transform.rotation = Quat::from_axis_angle(Vec3::Y, time.elapsed_seconds() * 2.0);

        for (player_entity, mut player_transform, mut player) in players.iter_mut() {
            if *vis != Visibility::Hidden {
                if (player_transform.translation.xz() - powerup_transform.translation.xz()).length()
                    < 1.0
                {
                    match powerup.powerup_type {
                        PowerupType::Health => {
                            player.health += 10.0;
                        }
                        PowerupType::Speed => {
                            player.speed += PLAYER_SPEED * 0.1;
                        }
                        PowerupType::Damage => {
                            player.damage += 0.15;
                        }
                    }
                    *vis = Visibility::Hidden;
                }
            }
        }
    }

    for event in events.read() {
        spawn_powerup(event.powerup_type, event.transform, &mut commands, &server);
    }
}

pub fn spawn_powerup(
    powerup_type: PowerupType,
    transform: Transform,
    commands: &mut Commands,
    server: &Res<AssetServer>,
) -> Entity {
    let scene = match &powerup_type {
        PowerupType::Health => server.load("food/apple.glb#Scene0"),
        PowerupType::Speed => server.load("food/banana.glb#Scene0"),
        PowerupType::Damage => server.load("food/cakeBirthday.glb#Scene0"),
    };

    commands
        .spawn(PowerupBundle {
            powerup: Powerup { powerup_type },
            scene: SceneBundle {
                scene: scene.clone(),
                transform: transform.with_scale(if powerup_type == PowerupType::Health {
                    Vec3::splat(5.0)
                } else {
                    Vec3::splat(3.0)
                }),
                ..default()
            },
        })
        .id()
}

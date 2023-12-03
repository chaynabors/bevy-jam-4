use bevy::prelude::*;

use crate::player::Player;

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

pub struct PowerupPlugin;

impl Plugin for PowerupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    mut commands: Commands,
    mut powerups: Query<(Entity, &mut Transform, &mut Powerup, &mut Visibility), Without<Player>>,
    mut players: Query<(Entity, &mut Transform, &mut Player), Without<Powerup>>,
    time: Res<Time>,
) {
    for (powerup_entity, mut powerup_transform, mut powerup, mut vis) in powerups.iter_mut() {
        for (player_entity, mut player_transform, mut player) in players.iter_mut() {
            if (player_transform.translation.xz() - powerup_transform.translation.xz()).length()
                < 1.0
            {
                match powerup.powerup_type {
                    PowerupType::Health => {
                        player.health = 100.0;
                    }
                    PowerupType::Speed => {
                        player.speed = 10.0;
                    }
                    PowerupType::Damage => {
                        player.damage = 2.0;
                    }
                }
                *vis = Visibility::Hidden;
            }
        }
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

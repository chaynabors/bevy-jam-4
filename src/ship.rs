use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_transforms);
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
}

impl Ship {
    pub fn new(max_speed: f32, acceleration_rate: f32) -> Self {
        Self {
            max_speed,
            acceleration_rate,
            ..Default::default()
        }
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }
}

fn update_transforms(time: Res<Time>, mut ships: Query<(&mut Ship, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut ship, mut transform) in &mut ships {
        transform.translation =
            transform.translation + ship.velocity * dt + 0.5 * ship.acceleration * dt * dt;
        let new_acceleration = ship.move_dir.clamp_length_max(1.0) * ship.acceleration_rate;
        ship.velocity = (ship.velocity + 0.5 * (ship.acceleration + new_acceleration) * dt)
            .clamp_length_max(ship.max_speed);
        ship.acceleration = new_acceleration;

        let look_dir = ship.look_dir.normalize_or_zero();
        if look_dir != Vec3::ZERO {
            transform.look_to(look_dir, Vec3::Y);
        }
    }
}

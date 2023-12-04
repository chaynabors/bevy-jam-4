use bevy::math::Vec3;

pub trait Vec3Extension {
    fn move_towards(self, target: Vec3, delta: f32) -> Vec3;
}

impl Vec3Extension for Vec3 {
    fn move_towards(self, target: Vec3, delta: f32) -> Vec3 {
        let to = target - self;
        let sqdist = to.length_squared();

        if sqdist == 0.0 || delta >= 0.0 && sqdist <= delta * delta {
            return target;
        }

        let dist = sqdist.sqrt();
        self + to / dist * delta
    }
}

use bevy::{prelude::*, render::camera::Camera, window::PrimaryWindow};

#[derive(Default, Resource)]
pub struct InputState {
    pub move_dir: Vec3,
    pub planar_cursor_position: Vec3,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputState::default())
            .add_systems(PreUpdate, update_input_state);
    }
}

fn update_input_state(
    keys: Res<Input<KeyCode>>,
    mut input: ResMut<InputState>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    input.move_dir = Vec3::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input.move_dir.z -= 1.0;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input.move_dir.z += 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input.move_dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input.move_dir.x += 1.0;
    }

    input.move_dir = input.move_dir.clamp_length_max(1.0);

    let window = window.single();
    let (camera, global_transform) = camera.single();

    let plane_origin = Vec3::new(0.0, 0.0, 0.0);
    let plane_normal = Vec3::new(0.0, 1.0, 0.0);

    let Some(viewport_position) = window.cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(global_transform, viewport_position) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(plane_origin, plane_normal) else {
        return;
    };

    input.planar_cursor_position = ray.get_point(distance)
}

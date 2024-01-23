use bevy::{input::mouse::*, prelude::*};

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
pub struct CameraTarget;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit_camera);
    }
}

fn orbit_camera(
    window_query: Query<&Window>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<CameraController>>,
    mut target_query: Query<&mut Transform, (With<CameraTarget>, Without<CameraController>)>,
) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("There should be one and only one camera with a CameraController");

    let target_transform = target_query
        .get_single_mut()
        .expect("There should be one and only one CameraTarget");

    let window = window_query.get_single().unwrap();

    let mut rotation_move = Vec2::ZERO;

    if input_mouse.pressed(MouseButton::Right) {
        for event in mouse_motion_event.read() {
            rotation_move += event.delta;
        }
    }

    if rotation_move.length_squared() > 0.0 {
        let delta_x = rotation_move.x / window.width() * std::f32::consts::PI * 2.0;
        let delta_y = rotation_move.y / window.height() * std::f32::consts::PI * 2.0;

        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        camera_transform.rotation = yaw * camera_transform.rotation;
        camera_transform.rotation *= pitch;

        let rot_matrix = Mat3::from_quat(camera_transform.rotation);
        camera_transform.translation = target_transform.translation
            + rot_matrix.mul_vec3(Vec3::new(
                0.0,
                0.0,
                (target_transform.translation - camera_transform.translation).length(),
            ));
    }

    mouse_motion_event.clear();
}

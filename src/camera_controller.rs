use bevy::{input::mouse::*, prelude::*};

#[derive(Component, Reflect)]
pub struct CameraController {
    pub yawn: f32,
    pub pitch: f32,
    pub radius: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub max_offset: Vec2,
    pub min_offset: Vec2,
    pub focus: Vec3,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub movement_smoothness: f32,
}

#[derive(Component)]
pub struct CameraTarget;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (orbit_camera, zoom_camera, sync_camera_with_target).chain(),
        )
        .register_type::<CameraController>();
    }
}

fn orbit_camera(
    window_query: Query<&Window>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    mouse_input: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut CameraController>,
) {
    let mut camera_controller = camera_query
        .get_single_mut()
        .expect("There should be one and only one camera with a CameraController");

    let window = window_query.get_single().unwrap();

    let mouse_delta = match mouse_input.pressed(MouseButton::Right) {
        true => mouse_motion_event
            .read()
            .map(|event| event.delta)
            .sum::<Vec2>(),
        false => Vec2::ZERO,
    };

    let Vec2 {
        x: delta_x,
        y: delta_y,
    } = mouse_delta / window.width()
        * camera_controller.mouse_sensitivity
        * std::f32::consts::PI
        * 2.0;

    camera_controller.yawn -= delta_x;
    camera_controller.pitch -= delta_y;

    mouse_motion_event.clear();
}

fn zoom_camera(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraController>,
) {
    let mut camera_controller = camera_query
        .get_single_mut()
        .expect("There should be one and only one camera with a CameraController");

    let scroll_delta = scroll_events.read().map(|event| -event.y).sum::<f32>();

    camera_controller.radius += scroll_delta * camera_controller.zoom_sensitivity;
    camera_controller.radius = camera_controller
        .radius
        .clamp(camera_controller.min_radius, camera_controller.max_radius);
}

fn sync_camera_with_target(
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
    target_query: Query<&GlobalTransform, (With<CameraTarget>, Without<CameraController>)>,
) {
    let (mut camera_transform, mut camera_controller) = camera_query
        .get_single_mut()
        .expect("There should be one and only one camera with a CameraController");

    let target_transform = target_query
        .get_single()
        .expect("There should be one and only one CameraTarget");

    let mut rotation = Quat::from_rotation_y(camera_controller.yawn);
    rotation *= Quat::from_rotation_x(camera_controller.pitch);

    let offset = {
        let percentage = (camera_controller.radius - camera_controller.min_radius)
            / (camera_controller.max_radius - camera_controller.min_radius);

        (camera_controller.max_offset * percentage)
            + (camera_controller.min_offset * (1.0 - percentage))
    };

    let right = camera_transform.rotation * Vec3::X * offset.x;
    let up = camera_transform.rotation * Vec3::Y * offset.y;
    let pan_translation = right + up;

    camera_transform.rotation = rotation;

    camera_controller.focus = camera_controller.focus
        + (target_transform.translation() - camera_controller.focus)
            * camera_controller.movement_smoothness;

    camera_transform.translation = camera_controller.focus
        + camera_transform.rotation * Vec3::new(0.0, 0.0, camera_controller.radius)
        + pan_translation;
}

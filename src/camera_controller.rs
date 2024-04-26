use bevy::{input::mouse::*, prelude::*};

use crate::state::GameState;

pub struct CameraControllerDescriptor {
    pub min_radius: f32,
    pub max_radius: f32,
    pub max_offset: Vec2,
    pub min_offset: Vec2,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub movement_smoothness: f32,
}

#[derive(Component, Reflect)]
pub struct CameraController {
    yawn: f32,
    pitch: f32,
    radius: f32,
    radius_target: f32,
    min_radius: f32,
    max_radius: f32,
    max_offset: Vec2,
    min_offset: Vec2,
    focus: Vec3,
    mouse_sensitivity: f32,
    zoom_sensitivity: f32,
    movement_smoothness: f32,
}

impl CameraController {
    pub fn new(descriptor: CameraControllerDescriptor) -> CameraController {
        let initial_radius =
            descriptor.min_radius + (descriptor.max_radius - descriptor.min_radius) / 2.0;

        CameraController {
            focus: Vec3::new(0.0, 0.0, 0.0),
            yawn: 0.0,
            pitch: 0.0,
            radius: initial_radius,
            radius_target: initial_radius,
            min_radius: descriptor.min_radius,
            max_radius: descriptor.max_radius,
            max_offset: descriptor.max_offset,
            min_offset: descriptor.min_offset,
            mouse_sensitivity: descriptor.mouse_sensitivity,
            zoom_sensitivity: descriptor.zoom_sensitivity,
            movement_smoothness: descriptor.movement_smoothness,
        }
    }
}

#[derive(Component)]
pub struct CameraTarget;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                orbit_camera,
                zoom_camera_with_scroll,
                sync_camera_with_target,
            )
                .chain(),
        )
        .add_systems(
            Update,
            focus_camera_with_right_mouse.run_if(in_state(GameState::InGame)),
        )
        .register_type::<CameraController>();
    }
}

fn orbit_camera(
    window_query: Query<&Window>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut camera_query: Query<&mut CameraController>,
    state: Res<State<GameState>>,
) {
    if state.get() == &GameState::InGame {
        let mut camera_controller = camera_query
            .get_single_mut()
            .expect("There should be one and only one camera with a CameraController");

        let window = window_query.get_single().unwrap();

        let mouse_delta = mouse_motion_event
            .read()
            .map(|event| event.delta)
            .sum::<Vec2>();

        let Vec2 {
            x: delta_x,
            y: delta_y,
        } = mouse_delta / window.width()
            * camera_controller.mouse_sensitivity
            * std::f32::consts::PI
            * 2.0;

        camera_controller.yawn -= delta_x;
        camera_controller.pitch -= delta_y;
    }
    mouse_motion_event.clear();
}

fn zoom_camera_with_scroll(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraController>,
    state: Res<State<GameState>>,
) {
    if state.get() == &GameState::InGame {
        let mut camera_controller = camera_query
            .get_single_mut()
            .expect("There should be one and only one camera with a CameraController");

        let scroll_delta = scroll_events.read().map(|event| -event.y).sum::<f32>();

        camera_controller.radius_target += scroll_delta * camera_controller.zoom_sensitivity;
        camera_controller.radius_target = camera_controller
            .radius_target
            .clamp(camera_controller.min_radius, camera_controller.max_radius);
    }
    scroll_events.clear();
}

fn focus_camera_with_right_mouse(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut CameraController>,
    mut previous_radius: Local<f32>,
) {
    let mut camera_controller = camera_query
        .get_single_mut()
        .expect("There should be one and only one camera with a CameraController");

    if mouse_input.just_pressed(MouseButton::Right) {
        *previous_radius = camera_controller.radius;
        camera_controller.radius_target = camera_controller.min_radius;
    } else if mouse_input.just_released(MouseButton::Right) {
        camera_controller.radius_target = *previous_radius;
    }
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

    camera_controller.radius = camera_controller.radius_target
        - (camera_controller.radius_target - camera_controller.radius) * 0.5;

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

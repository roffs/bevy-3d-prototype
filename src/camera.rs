use bevy::prelude::*;

use crate::camera_controller::{CameraController, CameraControllerDescriptor};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle::default(),
        CameraController::new(CameraControllerDescriptor {
            min_radius: 4.5,
            max_radius: 12.5,
            min_offset: Vec2::new(1.0, 0.7),
            max_offset: Vec2::new(3.0, 2.0),
            mouse_sensitivity: 1.0,
            zoom_sensitivity: 0.5,
            movement_smoothness: 0.05,
        }),
        Name::new("Main camera"),
    );

    commands.spawn(camera);
}

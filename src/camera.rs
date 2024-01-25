use bevy::prelude::*;

use crate::camera_controller::{CameraController, CameraTarget};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, player_query: Query<&Transform, With<CameraTarget>>) {
    let player_transform = player_query
        .get_single()
        .expect("Player has not been spawned yet.");

    let camera_offset = Vec3::new(2.0, 5.0, 5.0);

    let mut camera_transform = *player_transform;
    camera_transform.translation += camera_offset;
    camera_transform = camera_transform.looking_at(player_transform.translation, Vec3::Y);

    let radius = camera_offset.length();

    let yawn =
        (-camera_offset.z / (camera_offset.x.powi(2) + camera_offset.z.powi(2)).sqrt()).acos();
    let pitch = (-camera_offset.y / radius).asin();

    let camera = (
        Camera3dBundle {
            transform: camera_transform,
            ..default()
        },
        CameraController {
            yawn,
            pitch,
            radius,
            offset: (2.0, 1.0),
            focus: player_transform.translation,
            mouse_sensitivity: 1.0,
            scroll_sensitivity: 0.5,
            movement_smoothness: 0.05,
        },
        Name::new("Main camera"),
    );

    commands.spawn(camera);
}

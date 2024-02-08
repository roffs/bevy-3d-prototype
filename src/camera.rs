use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    camera_controller::{CameraController, CameraControllerDescriptor},
    state::GameState,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::InGame), lock_cursor)
            .add_systems(OnExit(GameState::InGame), unlock_cursor);
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
            mouse_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            movement_smoothness: 0.05,
        }),
        Name::new("Main camera"),
    );

    commands.spawn(camera);
}

fn lock_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn unlock_cursor(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}

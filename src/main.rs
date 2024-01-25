use bevy::prelude::*;

mod camera;
mod camera_controller;
mod player;
mod world;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use camera_controller::CameraControllerPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            CameraControllerPlugin,
            WorldPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}

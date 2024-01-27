use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new()) // egui integration
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            PlayerPlugin,
            CameraPlugin,
            CameraControllerPlugin,
            WorldPlugin,
        ))
        .run();
}

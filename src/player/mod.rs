use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

mod animation;
mod controller;

use crate::camera_controller::CameraTarget;
use animation::PlayerAnimationPlugin;
use controller::{
    ForwardDirection, MovementDirection, PlayerControllerBundle, PlayerControllerPlugin,
    PlayerState, VerticalSpeed,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HookPlugin)
            .add_plugins((PlayerControllerPlugin, PlayerAnimationPlugin))
            .add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        HookedSceneBundle {
            scene: SceneBundle {
                scene: assets.load("player.gltf#Scene0"),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                if entity.get::<Handle<Mesh>>().is_some() {
                    commands.insert(NoFrustumCulling);
                }
            }),
        },
        PlayerControllerBundle {
            initial_state: PlayerState::Idle,
            initial_forward_direction: ForwardDirection(Vec3::new(0.0, 0.0, -1.0)),
            movement_direction: MovementDirection(Vec3::new(0.0, 0.0, -1.0)),
            initial_vertical_speed: VerticalSpeed(0.0),
            collider: Collider::capsule(Vec3::new(0.0, 0.3, 0.0), Vec3::new(0.0, 1.5, 0.0), 0.3),
            kinematic_character_controller: KinematicCharacterController { ..default() },
        },
        Name::new("Player"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn((
            CameraTarget,
            TransformBundle {
                local: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
        ));
    });
}

use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use std::collections::HashMap;

mod controller;

use crate::camera_controller::CameraTarget;
use controller::PlayerControllerPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HookPlugin)
            .add_plugins(PlayerControllerPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (setup_scene_once_loaded, keyboard_animation_control),
            );
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Animation {
    Idle,
    Walk,
    Run,
    Sprint,
    Jump,
}

#[derive(Resource)]
struct AnimationHandles(HashMap<Animation, Handle<AnimationClip>>);

impl AnimationHandles {
    pub fn get(&self, animation: &Animation) -> Handle<AnimationClip> {
        self.0.get(animation).unwrap().clone_weak()
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    #[rustfmt::skip]
    commands.insert_resource(AnimationHandles(HashMap::from([
        (Animation::Idle, assets.load("player.gltf#Animation0")),
        (Animation::Jump, assets.load("player.gltf#Animation1")),
        (Animation::Run, assets.load("player.gltf#Animation2")),
        (Animation::Sprint, assets.load("player.gltf#Animation3")),
        (Animation::Walk, assets.load("player.gltf#Animation4")),
    ])));

    let player = (
        HookedSceneBundle {
            scene: SceneBundle {
                scene: assets.load("player.gltf#Scene0"),
                // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                if entity.get::<Handle<Mesh>>().is_some() {
                    commands.insert(NoFrustumCulling);
                }
            }),
        },
        Name::new("Player"),
        Collider::capsule(Vec3::new(0.0, 0.3, 0.0), Vec3::new(0.0, 1.5, 0.0), 0.3),
        KinematicCharacterController { ..default() },
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

fn setup_scene_once_loaded(
    animations: Res<AnimationHandles>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.get(&Animation::Idle)).repeat();
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<AnimationHandles>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut animation_players {
        if keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::S)
            || keyboard_input.pressed(KeyCode::D)
        {
            if *current_animation != 1 {
                *current_animation = 1;
                player.play(animations.get(&Animation::Walk)).repeat();
            }
        } else if keyboard_input.pressed(KeyCode::Space) {
            if *current_animation != 2 {
                *current_animation = 2;
                player.play(animations.get(&Animation::Jump));
            }
        } else if *current_animation == 1 || (*current_animation == 2 && player.is_finished()) {
            *current_animation = 0;
            player.play(animations.get(&Animation::Idle)).repeat();
        }
    }
}

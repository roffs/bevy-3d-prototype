use bevy::prelude::*;

use crate::camera_controller::CameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement,
                setup_scene_once_loaded,
                keyboard_animation_control,
            ),
        );
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn player_movement(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_transform, player_speed) = player_query.get_single_mut().unwrap();
    let camera_transform = camera_query.get_single().unwrap();

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        direction += camera_transform.forward();
    }

    if keys.pressed(KeyCode::S) {
        direction += camera_transform.back();
    }

    if keys.pressed(KeyCode::D) {
        direction += camera_transform.right();
    }

    if keys.pressed(KeyCode::A) {
        direction += camera_transform.left();
    }

    direction.y = 0.0;

    if direction != Vec3::ZERO {
        let movement = direction.normalize() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
        player_transform.look_to(-direction, Vec3::Y);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![
        assets.load("player.gltf#Animation0"), // idle
        assets.load("player.gltf#Animation1"), // jump
        assets.load("player.gltf#Animation2"), // walk
    ]));

    let player = (
        SceneBundle {
            scene: assets.load("player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Speed(1.7),
        Player,
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

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
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
                player.play(animations.0[2].clone_weak()).repeat();
            }
        } else if keyboard_input.pressed(KeyCode::Space) {
            if *current_animation != 2 {
                *current_animation = 2;
                player.play(animations.0[1].clone_weak());
            }
        } else if *current_animation == 1 || (*current_animation == 2 && player.is_finished()) {
            *current_animation = 0;
            player.play(animations.0[0].clone_weak()).repeat();
        }
    }
}

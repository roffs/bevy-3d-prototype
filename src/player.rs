use bevy::prelude::*;

use crate::camera_controller::CameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

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

    let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
    player_transform.translation += movement;
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Speed(4.0),
        Player,
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

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::schedule::InGameSet;

#[derive(Component)]
pub struct MovementDirection(pub Vec3);

#[derive(Component)]
pub struct ForwardDirection(pub Vec3);

#[derive(Component)]
pub struct VerticalSpeed(pub f32);

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Runing,
    Sprinting,
    Jumping,
}

#[derive(Bundle)]
pub struct PlayerControllerBundle {
    pub initial_state: PlayerState,
    pub initial_forward_direction: ForwardDirection,
    pub movement_direction: MovementDirection,
    pub initial_vertical_speed: VerticalSpeed,
    pub collider: Collider,
    pub kinematic_character_controller: KinematicCharacterController,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player_state, update_player_forward)
                .chain()
                .in_set(InGameSet::UserInput),
        )
        .add_systems(Update, move_player.in_set(InGameSet::EntityUpdates));
    }
}

const WALKING_SPEED: f32 = 1.8;
const RUNNING_SPEED: f32 = 4.0;

fn update_player_state(
    mut player_query: Query<(
        &mut PlayerState,
        &mut MovementDirection,
        &ForwardDirection,
        &KinematicCharacterControllerOutput,
        &mut VerticalSpeed,
    )>,
    keys: Res<Input<KeyCode>>,
) {
    for (
        mut player_state,
        mut target_direction,
        forward_direction,
        controller,
        mut vertical_speed,
    ) in player_query.iter_mut()
    {
        let forward = forward_direction.0;
        let right = forward.cross(Vec3::Y);

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += forward;
        }

        if keys.pressed(KeyCode::S) {
            direction -= forward;
        }

        if keys.pressed(KeyCode::D) {
            direction += right;
        }

        if keys.pressed(KeyCode::A) {
            direction -= right;
        }

        if controller.grounded {
            vertical_speed.0 = -3.0;
            if keys.pressed(KeyCode::Space) {
                *player_state = PlayerState::Jumping;
            } else if direction != Vec3::ZERO {
                if keys.pressed(KeyCode::ShiftLeft) {
                    *player_state = PlayerState::Runing;
                } else {
                    *player_state = PlayerState::Walking;
                }
                *target_direction = MovementDirection(direction.normalize());
            } else {
                *player_state = PlayerState::Idle;
            }
        }
    }
}

fn update_player_forward(
    mut player_query: Query<&mut ForwardDirection>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<KinematicCharacterController>)>,
) {
    let mut forward_direction = player_query.single_mut();
    let camera_transform = camera_query.get_single().unwrap();

    let mut target_direction = camera_transform.forward();
    target_direction.y = 0.0;

    forward_direction.0 = target_direction.normalize();
}

fn move_player(
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut Transform,
        &PlayerState,
        &MovementDirection,
        &mut VerticalSpeed,
    )>,
    time: Res<Time>,
) {
    for (mut controller, mut transform, player_state, target_direction, mut vertical_speed) in
        controllers.iter_mut()
    {
        let speed = match player_state {
            PlayerState::Walking => WALKING_SPEED,
            PlayerState::Runing => RUNNING_SPEED,
            _ => 0.0,
        };

        let vertical_movement = Vec3::Y * vertical_speed.0 * time.delta_seconds();

        let movement = target_direction.0 * speed * time.delta_seconds();
        controller.translation = Some(movement + vertical_movement);

        transform.look_to(-target_direction.0, Vec3::Y);

        let gravity = 10.0;
        vertical_speed.0 -= gravity * time.delta_seconds();
    }
}

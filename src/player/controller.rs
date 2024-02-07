use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{camera_controller::CameraController, schedule::InGameSet};

#[derive(Component)]
pub struct MovementDirection(pub Vec3);

#[derive(Component)]
pub struct VerticalSpeed(pub f32);

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Runing,
    Sprinting,
    Jumping,
    Aiming,
}

#[derive(Bundle)]
pub struct PlayerControllerBundle {
    pub initial_state: PlayerState,
    pub movement_direction: MovementDirection,
    pub initial_vertical_speed: VerticalSpeed,
    pub collider: Collider,
    pub kinematic_character_controller: KinematicCharacterController,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_state.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (apply_gravity, move_player)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

const WALKING_SPEED: f32 = 1.8;
const RUNNING_SPEED: f32 = 4.0;

fn update_player_state(
    mut player_query: Query<(
        &mut PlayerState,
        &KinematicCharacterControllerOutput,
        &mut MovementDirection,
    )>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    for (mut player_state, controller, mut movement_direction) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += Vec3::X;
        }

        if keys.pressed(KeyCode::S) {
            direction -= Vec3::X;
        }

        if keys.pressed(KeyCode::D) {
            direction += Vec3::Z;
        }

        if keys.pressed(KeyCode::A) {
            direction -= Vec3::Z;
        }

        movement_direction.0 = direction;

        if !controller.grounded {
            return;
        }

        if keys.pressed(KeyCode::Space) {
            *player_state = PlayerState::Jumping;
        } else if direction != Vec3::ZERO {
            if keys.pressed(KeyCode::ShiftLeft) {
                *player_state = PlayerState::Runing;
            } else {
                *player_state = PlayerState::Walking;
            }
        } else if mouse.pressed(MouseButton::Right) {
            *player_state = PlayerState::Aiming;
        } else {
            *player_state = PlayerState::Idle;
        }
    }
}

fn move_player(
    mut controller_query: Query<(
        &mut KinematicCharacterController,
        &mut Transform,
        &PlayerState,
        &MovementDirection,
        &mut VerticalSpeed,
    )>,
    camera_query: Query<
        &Transform,
        (
            With<CameraController>,
            Without<KinematicCharacterController>,
        ),
    >,

    time: Res<Time>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("There should be one and only one camera with a CameraController");

    for (mut controller, mut player_transform, player_state, movement_direction, vertical_speed) in
        controller_query.iter_mut()
    {
        let mut forward = player_transform.translation - camera_transform.translation;
        forward.y = 0.0;
        let forward = forward.normalize();
        let right = forward.cross(Vec3::Y);

        let speed = match player_state {
            PlayerState::Walking => WALKING_SPEED,
            PlayerState::Runing => RUNNING_SPEED,
            _ => 0.0,
        };

        let direction = forward * movement_direction.0.x + right * movement_direction.0.z;
        let vertical_movement = Vec3::Y * vertical_speed.0 * time.delta_seconds();

        let movement = direction * speed * time.delta_seconds();
        controller.translation = Some(movement + vertical_movement);

        match player_state {
            PlayerState::Aiming => player_transform.look_to(-forward, Vec3::Y),
            _ => player_transform.look_to(-direction, Vec3::Y),
        }
    }
}

fn apply_gravity(
    mut controller_query: Query<(&KinematicCharacterControllerOutput, &mut VerticalSpeed)>,
    time: Res<Time>,
) {
    for (controller, mut vertical_speed) in controller_query.iter_mut() {
        let gravity = 9.8;
        match controller.grounded {
            true => vertical_speed.0 = 0.0,
            false => vertical_speed.0 -= gravity * time.delta_seconds(),
        }
    }
}

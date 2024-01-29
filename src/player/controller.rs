use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::TargetDirection;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_player_state, move_player));
    }
}

const WALKING_SPEED: f32 = 1.8;
const RUNNING_SPEED: f32 = 4.0;

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub(super) enum PlayerState {
    Idle,
    Walk,
    Run,
    Sprint,
    Jump,
}

fn update_player_state(
    mut player_query: Query<(&mut PlayerState, &mut TargetDirection)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<KinematicCharacterController>)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut player_state, mut target_direction) = player_query.single_mut();
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
        if keys.pressed(KeyCode::ShiftLeft) {
            *player_state = PlayerState::Run;
        } else {
            *player_state = PlayerState::Walk;
        }
        *target_direction = TargetDirection(direction.normalize());
    } else {
        *player_state = PlayerState::Idle;
    }
}

fn move_player(
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut Transform,
        &PlayerState,
        &TargetDirection,
    )>,
    time: Res<Time>,
) {
    for (mut controller, mut transform, player_state, target_direction) in controllers.iter_mut() {
        let speed = match player_state {
            PlayerState::Walk => WALKING_SPEED,
            PlayerState::Run => RUNNING_SPEED,
            _ => 0.0,
        };

        let gravity = -Vec3::Y * time.delta_seconds();
        let movement = target_direction.0 * speed * time.delta_seconds();
        controller.translation = Some(movement + gravity);

        transform.look_to(-target_direction.0, Vec3::Y);
    }
}

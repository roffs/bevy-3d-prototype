use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct MovementDirection(pub Vec3);

#[derive(Component)]
pub struct ForwardDirection(pub Vec3);

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub enum PlayerState {
    Idle,
    Walk,
    Run,
    Sprint,
    Jump,
}

#[derive(Bundle)]
pub struct PlayerControllerBundle {
    pub initial_state: PlayerState,
    pub initial_forward_direction: ForwardDirection,
    pub movement_direction: MovementDirection,
    pub collider: Collider,
    pub kinematic_character_controller: KinematicCharacterController,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player_state, update_player_forward, move_player).chain(),
        );
    }
}

const WALKING_SPEED: f32 = 1.8;
const RUNNING_SPEED: f32 = 4.0;

fn update_player_state(
    mut player_query: Query<(&mut PlayerState, &mut MovementDirection, &ForwardDirection)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut player_state, mut target_direction, forward_direction) = player_query.single_mut();

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

    direction.y = 0.0;

    if direction != Vec3::ZERO {
        if keys.pressed(KeyCode::ShiftLeft) {
            *player_state = PlayerState::Run;
        } else {
            *player_state = PlayerState::Walk;
        }
        *target_direction = MovementDirection(direction.normalize());
    } else {
        *player_state = PlayerState::Idle;
    }
}

fn update_player_forward(
    mut player_query: Query<&mut ForwardDirection>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<KinematicCharacterController>)>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        let mut forward_direction = player_query.single_mut();
        let camera_transform = camera_query.get_single().unwrap();

        let mut target_direction = camera_transform.forward();
        target_direction.y = 0.0;

        forward_direction.0 = target_direction.normalize();
    }
}

fn move_player(
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut Transform,
        &PlayerState,
        &MovementDirection,
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

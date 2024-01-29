use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

const WALKING_SPEED: f32 = 1.8;
const RUNNING_SPEED: f32 = 4.0;

fn move_player(
    mut controllers: Query<(&mut KinematicCharacterController, &mut Transform)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<KinematicCharacterController>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let camera_transform = camera_query.get_single().unwrap();

    for (mut controller, mut transform) in controllers.iter_mut() {
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

        let gravity = -Vec3::Y * time.delta_seconds();
        let movement = direction.normalize() * WALKING_SPEED * time.delta_seconds();
        controller.translation = Some(movement + gravity);

        if direction != Vec3::ZERO {
            transform.look_to(-direction, Vec3::Y);
        }
    }
}

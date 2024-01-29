use bevy::prelude::*;

use std::collections::HashMap;

use super::PlayerState;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_animations_resource)
            .add_systems(Update, update_animation);
    }
}

#[derive(Resource)]
struct AnimationHandles(HashMap<PlayerState, Handle<AnimationClip>>);

impl AnimationHandles {
    pub fn get(&self, animation: &PlayerState) -> Handle<AnimationClip> {
        self.0.get(animation).unwrap().clone_weak()
    }
}

fn initialize_animations_resource(mut commands: Commands, assets: Res<AssetServer>) {
    #[rustfmt::skip]
    commands.insert_resource(AnimationHandles(HashMap::from([
        (PlayerState::Idle, assets.load("player.gltf#Animation0")),
        (PlayerState::Jump, assets.load("player.gltf#Animation1")),
        (PlayerState::Run, assets.load("player.gltf#Animation2")),
        (PlayerState::Sprint, assets.load("player.gltf#Animation3")),
        (PlayerState::Walk, assets.load("player.gltf#Animation4")),
    ])));
}

fn update_animation(
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<AnimationHandles>,
    player_state_query: Query<&PlayerState>,
) {
    let player_state = player_state_query.single();

    for mut player in &mut animation_players {
        player.play(animations.get(player_state)).repeat();
    }
}
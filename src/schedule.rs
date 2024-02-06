use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (InGameSet::UserInput, InGameSet::EntityUpdates)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

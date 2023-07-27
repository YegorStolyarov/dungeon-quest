use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod cheat;
pub mod cleanup;
pub mod feature;
pub mod movement;

pub struct InputHandlePlugin;


impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGameClassicMode), cleanup::cleanup_mouse);
        app.add_systems(OnEnter(SceneState::InGameSurvivalMode), cleanup::cleanup_mouse);

        app.add_systems(Update, (
            feature::use_skill,
            feature::pause,
            feature::use_mouse,
            movement::player_movement_handle_system.after(crate::plugins::player::stats::update_stats)
        ).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

        app.add_systems(Update, cheat::unlock_room_cheat.run_if(in_state(SceneState::InGameClassicMode)));

        app.add_systems(Update, (
            cheat::knight_skill_cheat,
            cheat::damage_player_cheat
        ).run_if(in_state(SceneState::InGameSurvivalMode)));
    }
}

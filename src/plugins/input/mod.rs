use bevy::prelude::*;
use crate::scenes::pause_scene::PauseSceneData;

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

        app.add_systems(Update,
            cleanup::cleanup_mouse.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode)).and_then(resource_removed::<PauseSceneData>()))
        );

        app.add_systems(Update, (
            feature::use_skill,
            crate::scenes::pause_scene::pause,
            feature::use_mouse,
            movement::player_movement_handle_system.after(crate::plugins::player::stats::update_stats)
        ).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode)).and_then(not(resource_exists::<PauseSceneData>())))
        );

        app.add_systems(Update, crate::scenes::pause_scene::button_handle_system.run_if(resource_exists::<PauseSceneData>()));

        app.add_systems(Update, cheat::unlock_room_cheat.run_if(in_state(SceneState::InGameClassicMode)));

        app.add_systems(Update, (
            cheat::knight_skill_cheat,
            cheat::damage_player_cheat
        ).run_if(in_state(SceneState::InGameSurvivalMode)));
    }
}

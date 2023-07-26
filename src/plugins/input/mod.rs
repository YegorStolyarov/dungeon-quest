use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod cheat;
pub mod cleanup;
pub mod feature;
pub mod movement;

pub struct InputHandlePlugin;


impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cleanup::cleanup_mouse.in_schedule(OnEnter(SceneState::InGameClassicMode)));

        // app.add_system_set(
        //     SystemSet::on_resume(SceneState::InGameClassicMode).with_system(cleanup::cleanup_mouse),
        // );

        app.add_system(feature::pause.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(feature::use_skill.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(feature::use_mouse.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(cheat::unlock_room_cheat.in_set(OnUpdate(SceneState::InGameClassicMode)));

        app.add_system(cleanup::cleanup_mouse.in_schedule(OnEnter(SceneState::InGameSurvivalMode)));

        // app.add_system_set(
        //     SystemSet::on_resume(SceneState::InGameSurvivalMode)
        //         .with_system(cleanup::cleanup_mouse),
        // );

        app.add_system(feature::use_skill.in_set(OnUpdate(SceneState::InGameSurvivalMode)));
        app.add_system(feature::pause.in_set(OnUpdate(SceneState::InGameSurvivalMode)));
        app.add_system(feature::use_mouse.in_set(OnUpdate(SceneState::InGameSurvivalMode)));
        app.add_system(cheat::knight_skill_cheat.in_set(OnUpdate(SceneState::InGameSurvivalMode)));
        app.add_system(cheat::damage_player_cheat.in_set(OnUpdate(SceneState::InGameSurvivalMode)));

        app.add_system(movement::player_movement_handle_system.after(crate::plugins::player::stats::update_stats).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

    }
}

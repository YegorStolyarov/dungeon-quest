use bevy::prelude::*;

use crate::scenes::SceneState;

mod animation;
mod cleanup;
mod effect;
mod invinsible;
mod movement;
mod spawn;

pub struct MonsterPlugin;



impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation::animation_handle.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(movement::move_to_player.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(movement::change_direction.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(effect::update_effects.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(effect::update_color_of_effects.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(cleanup::cleanup_killed_monsters.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

        app.add_system(cleanup::cleanup_monster_after_cleared_room.in_set(OnUpdate(SceneState::InGameClassicMode)));

        app.add_system(spawn::spawn_monsters_classic_mode.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(spawn::spawn_monsters_survival_mode.in_set(OnUpdate(SceneState::InGameSurvivalMode)));

        app.add_system(cleanup::cleanup_monsters.in_schedule(OnExit(SceneState::InGameClassicMode)));

        app.add_system(cleanup::cleanup_monsters.in_schedule(OnExit(SceneState::InGameSurvivalMode)));

        app.add_system(invinsible::hurt_duration_color.after(effect::update_color_of_effects).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
    }
}

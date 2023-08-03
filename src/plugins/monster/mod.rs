use bevy::prelude::*;
use crate::resources::game_data::PauseSceneData;

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
        app.add_systems(Update, (
            animation::animation_handle,
            movement::move_to_player,
            effect::update_effects,
            movement::change_direction,
            effect::update_color_of_effects,
            cleanup::cleanup_killed_monsters,
            invinsible::hurt_duration_color.after(effect::update_color_of_effects)
        ).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode)).and_then(not(resource_exists::<PauseSceneData>()))));

        app.add_systems(Update, (
            spawn::spawn_monsters_classic_mode,
            cleanup::cleanup_monster_after_cleared_room
        ).run_if(in_state(SceneState::InGameClassicMode)));

        app.add_systems(Update,spawn::spawn_monsters_survival_mode
            .run_if(in_state(SceneState::InGameSurvivalMode)));

        app.add_systems(OnExit(SceneState::InGameClassicMode), cleanup::cleanup_monsters);
        app.add_systems(OnExit(SceneState::InGameSurvivalMode), cleanup::cleanup_monsters);

    }
}

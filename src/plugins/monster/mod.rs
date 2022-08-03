use bevy::prelude::*;

use crate::scenes::SceneState;

mod animation;
mod cleanup;
mod invinsible;
mod movement;
mod spawn;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(spawn::spawn_monsters_classic_mode)
                .with_system(animation::animation_handle)
                .with_system(movement::move_to_player)
                .with_system(movement::change_direction)
                .with_system(invinsible::hurt_duration_color)
                .with_system(cleanup::cleanup_killed_monsters)
                .with_system(cleanup::cleanup_monster_after_cleared_room),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicMode)
                .with_system(cleanup::cleanup_monsters),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(spawn::spawn_monsters_survival_mode)
                .with_system(animation::animation_handle)
                .with_system(movement::move_to_player)
                .with_system(movement::change_direction)
                .with_system(invinsible::hurt_duration_color)
                .with_system(cleanup::cleanup_killed_monsters),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode)
                .with_system(cleanup::cleanup_monsters),
        );
    }
}

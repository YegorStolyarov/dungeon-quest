use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod dungeon;
pub mod interactions;

pub struct ClassicModePlugin;

impl Plugin for ClassicModePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PreInGameScene)
                .with_system(dungeon::initiate::initiate_classic_mode.label("Initiate")),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicModeScene)
                .with_system(dungeon::ground::ground)
                .with_system(dungeon::doors::doors)
                .with_system(dungeon::walls::walls)
                .with_system(dungeon::treasure::treasure),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicModeScene)
                .with_system(dungeon::doors::horizontal_doors_system)
                .with_system(dungeon::doors::vertical_doors_system)
                .with_system(dungeon::walls::temporary_walls_system)
                .with_system(dungeon::ground::ladder_system)
                .with_system(dungeon::treasure::treasure_system)
                .with_system(interactions::door::horizontal_door_interaction_handle)
                .with_system(interactions::door::vertical_door_interaction_handle)
                .with_system(interactions::ladder::ladder_interaction_handle_system)
                .with_system(interactions::treasure::treasure_interaction_handle_system),
        );
    }
}

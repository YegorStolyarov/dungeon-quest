use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod dungeon;
pub mod interaction;

pub struct ClassicModePlugin;

impl Plugin for ClassicModePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicModeScene)
                .with_system(dungeon::initiate::initiate_dungeon.label("Initiate"))
                .with_system(dungeon::layer::layer.after("Initiate"))
                .with_system(dungeon::treasure::treasure),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicModeScene)
                .with_system(dungeon::walls::clear_walls)
                .with_system(dungeon::walls::walls)
                .with_system(dungeon::doors::clear_doors)
                .with_system(dungeon::doors::doors)
                .with_system(dungeon::layer::change_floor_to_ladder)
                .with_system(dungeon::treasure::enable_treasure)
                .with_system(interaction::door_interaction_handle_system)
                .with_system(interaction::ladder_interaction_handle_system)
                .with_system(interaction::treasure_interaction_handle_system),
        );
    }
}

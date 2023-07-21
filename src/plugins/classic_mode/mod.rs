use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod dungeon;
pub mod interactions;
pub mod ui;

pub struct ClassicModePlugin;

#[derive(Resource)]
pub struct ClassicModeData {
    pub doors: Option<Entity>,
    pub ground: Option<Entity>,
    pub walls: Option<Entity>,
    pub end_point: Option<Entity>,
}

impl Plugin for ClassicModePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PreClassicMode)
                .with_system(dungeon::initiate::initiate_classic_mode.label("Initiate")),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicMode)
                .with_system(dungeon::ground::ground)
                .with_system(dungeon::doors::doors)
                .with_system(dungeon::walls::walls)
                .with_system(dungeon::end_point::end_point),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(dungeon::doors::horizontal_doors_system)
                .with_system(dungeon::doors::vertical_doors_system)
                .with_system(dungeon::walls::temporary_walls_system)
                .with_system(dungeon::end_point::end_point_handle_system)
                .with_system(interactions::door::horizontal_door_interaction_handle)
                .with_system(interactions::door::vertical_door_interaction_handle)
                .with_system(interactions::end_point::end_point_interaction_handle_system)
                .with_system(interactions::unlock_room::cleared_room_check),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicMode).with_system(clean_up_classic_mode),
        );
    }
}

fn clean_up_classic_mode(mut commands: Commands, classic_mode_data: Res<ClassicModeData>) {
    commands
        .entity(classic_mode_data.doors.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.walls.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.end_point.unwrap())
        .despawn_recursive();

    commands
        .entity(classic_mode_data.ground.unwrap())
        .despawn_recursive();
}

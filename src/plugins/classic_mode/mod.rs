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
        // label("Initiate") - removed because unused
        app.add_system(dungeon::initiate::initiate_classic_mode.in_schedule(OnEnter(SceneState::PreClassicMode)));

        app.add_systems((
            dungeon::ground::ground,
            dungeon::doors::doors,
            dungeon::walls::walls,
            dungeon::end_point::end_point
        ).in_schedule(OnEnter(SceneState::InGameClassicMode)));

        app.add_system(dungeon::doors::horizontal_doors_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(dungeon::doors::vertical_doors_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(dungeon::walls::temporary_walls_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(dungeon::end_point::end_point_handle_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(interactions::door::horizontal_door_interaction_handle.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(interactions::door::vertical_door_interaction_handle.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(interactions::end_point::end_point_interaction_handle_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(interactions::unlock_room::cleared_room_check.in_set(OnUpdate(SceneState::InGameClassicMode)));

        app.add_system(clean_up_classic_mode.in_schedule(OnExit(SceneState::InGameClassicMode)));
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

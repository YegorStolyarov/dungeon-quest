use bevy::prelude::*;
use crate::resources::game_data::PauseSceneData;

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
        app.add_systems(OnEnter(SceneState::PreClassicMode), dungeon::initiate::initiate_classic_mode);

        app.add_systems(OnEnter(SceneState::InGameClassicMode), (
            dungeon::ground::ground,
            dungeon::doors::doors,
            dungeon::walls::walls,
            dungeon::end_point::end_point
        ));

        app.add_systems(Update, (
            dungeon::doors::horizontal_doors_system,
            dungeon::doors::vertical_doors_system,
            dungeon::walls::temporary_walls_system,
            dungeon::end_point::end_point_handle_system,
            interactions::door::horizontal_door_interaction_handle,
            interactions::door::vertical_door_interaction_handle,
            interactions::end_point::end_point_interaction_handle_system,
            interactions::unlock_room::cleared_room_check
        ).run_if(in_state(SceneState::InGameClassicMode).and_then(not(resource_exists::<PauseSceneData>()))));

        app.add_systems(Update, (
            interactions::end_point::cooldown_handle,
            interactions::end_point::collect_reward
        ).run_if(in_state(SceneState::InGameClassicMode).and_then(resource_exists::<interactions::end_point::RewardSceneFlag>())));

        app.add_systems(OnExit(SceneState::InGameClassicMode), clean_up_classic_mode);
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

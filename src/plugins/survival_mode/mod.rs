use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod dungeon;
pub mod ui;

#[derive(Resource)]
pub struct SurvivalModeData {
    pub ground: Option<Entity>,
    pub walls: Option<Entity>,
}
pub struct SurvivalModePlugin;

impl Plugin for SurvivalModePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dungeon::initiate::initiate_survival_mode.in_schedule(OnEnter(SceneState::PreSurvivalMode)));

        app.add_system(dungeon::ground::ground.in_schedule(OnEnter(SceneState::InGameSurvivalMode)));
        app.add_system(dungeon::walls::walls.in_schedule(OnEnter(SceneState::InGameSurvivalMode)));

        app.add_system(dungeon::wave::countdown.in_set(OnUpdate(SceneState::InGameSurvivalMode)));

        app.add_system(cleanup_survival_mode_data.in_schedule(OnExit(SceneState::InGameSurvivalMode)));
    }
}

fn cleanup_survival_mode_data(mut commands: Commands, survival_mode_data: Res<SurvivalModeData>) {
    commands
        .entity(survival_mode_data.walls.unwrap())
        .despawn_recursive();

    commands
        .entity(survival_mode_data.ground.unwrap())
        .despawn_recursive();
}

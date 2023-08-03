use bevy::prelude::*;
use crate::resources::game_data::PauseSceneData;

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
        app.add_systems(OnEnter(SceneState::PreSurvivalMode), dungeon::initiate::initiate_survival_mode);

        app.add_systems(OnEnter(SceneState::InGameSurvivalMode), (
            dungeon::ground::ground,
            dungeon::walls::walls
        ));

        app.add_systems(Update, dungeon::wave::countdown.run_if(
            in_state(SceneState::InGameSurvivalMode).and_then(not(resource_exists::<PauseSceneData>())
        )));

        app.add_systems(OnExit(SceneState::InGameSurvivalMode), cleanup_survival_mode_data);
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

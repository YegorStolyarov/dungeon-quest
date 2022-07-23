use bevy::prelude::*;

use crate::scenes::SceneState;

pub mod dungeon;

pub struct SurvivalModeData {
    pub ground: Option<Entity>,
    pub walls: Option<Entity>,
}
pub struct SurvivalModePlugin;

impl Plugin for SurvivalModePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PreSurvivalMode)
                .with_system(dungeon::initiate::initiate_survival_mode),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameSurvivalMode)
                .with_system(dungeon::ground::ground)
                .with_system(dungeon::walls::walls),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode).with_system(clean_up_classic_mode),
        );
    }
}

fn clean_up_classic_mode(mut commands: Commands, survival_mode_data: Res<SurvivalModeData>) {
    commands
        .entity(survival_mode_data.walls.unwrap())
        .despawn_recursive();

    commands
        .entity(survival_mode_data.ground.unwrap())
        .despawn_recursive();
}

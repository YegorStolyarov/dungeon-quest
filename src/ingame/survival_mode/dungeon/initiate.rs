use bevy::prelude::*;

use crate::ingame::resources::dungeon::wave::Wave;
use crate::ingame::survival_mode::SurvivalModeData;
use crate::scenes::SceneState;

pub fn initiate_survival_mode(mut commands: Commands, mut state: ResMut<State<SceneState>>) {
    let dungeon_mode_data = SurvivalModeData {
        walls: None,
        ground: None,
    };

    let wave = Wave::new();

    commands.insert_resource(dungeon_mode_data);
    commands.insert_resource(wave);

    state
        .set(SceneState::InGameSurvivalMode)
        .expect("Can't change to Ingame Classic Mode");
}

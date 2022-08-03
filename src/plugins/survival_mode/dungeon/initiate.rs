use bevy::prelude::*;

use crate::plugins::survival_mode::SurvivalModeData;
use crate::resources::dungeon::wave::Wave;
use crate::resources::game_mode::GameMode;
use crate::resources::monster::monster_spawn_controller::MonsterSpawnController;
use crate::scenes::SceneState;

pub fn initiate_survival_mode(mut commands: Commands, mut state: ResMut<State<SceneState>>) {
    let dungeon_mode_data = SurvivalModeData {
        walls: None,
        ground: None,
    };

    let wave = Wave::new();

    commands.insert_resource(dungeon_mode_data);
    commands.insert_resource(wave);

    let monster_spawn_controller = MonsterSpawnController {
        game_mode: GameMode::SurvivalMode,
        max_avalible_monsters: 15,
        require_monster: 0,
        alive_monsters: 0,
    };

    state
        .set(SceneState::InGameSurvivalMode)
        .expect("Can't change to Ingame Classic Mode");
}

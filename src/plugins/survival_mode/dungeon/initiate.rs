use bevy::prelude::*;

use crate::config::*;
use crate::plugins::survival_mode::dungeon::{TOTAL_TILE_HEIGHT, TOTAL_TILE_WIDTH};
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

    let start_x = 0.0 - (TOTAL_TILE_WIDTH * TILE_SIZE / 2.0 - TILE_SIZE / 2.0);
    let start_y = 0.0 + (TOTAL_TILE_HEIGHT * TILE_SIZE / 2.0 - TILE_SIZE / 2.0);

    let spawn_area_start_x = start_x + TILE_SIZE + TILE_SIZE * 2.0;
    let spawn_area_start_y = start_y - TILE_SIZE - TILE_SIZE * 2.0;
    let spawn_area_end_x = start_x + (TOTAL_TILE_WIDTH as f32 - 2.0) * TILE_SIZE;
    let spawn_area_end_y = start_y - (TOTAL_TILE_HEIGHT as f32 - 2.0) * TILE_SIZE;

    let monster_spawn_controller = MonsterSpawnController {
        game_mode: GameMode::SurvivalMode,
        max_avalible_monsters: 8,
        require_monster: 0,
        killed_monsters: 0,
        alive_monsters: 0,
        spawn_area_start_x,
        spawn_area_start_y,
        spawn_area_end_x,
        spawn_area_end_y,
    };

    commands.insert_resource(monster_spawn_controller);
    commands.insert_resource(dungeon_mode_data);
    commands.insert_resource(wave);

    state
        .set(SceneState::InGameSurvivalMode)
        .expect("Can't change to Ingame Classic Mode");
}

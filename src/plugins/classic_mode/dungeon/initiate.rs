use bevy::prelude::*;

use crate::config::*;
use crate::plugins::classic_mode::ClassicModeData;
use crate::resources::dungeon::Dungeon;
use crate::resources::game_mode::GameMode;
use crate::resources::monster::monster_spawn_controller::MonsterSpawnController;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::scenes::SceneState;

pub fn initiate_classic_mode(mut commands: Commands, mut state: ResMut<NextState<SceneState>>) {
    let dungeon = Dungeon::new();

    let player_dungeon_stats = PlayerDungeonStats {
        current_room_position: dungeon.current_floor.current_position,
        is_room_cleared: true,
        current_floor_index: 0,
    };

    let classic_mode_data = ClassicModeData {
        doors: None,
        walls: None,
        ground: None,
        end_point: None,
    };

    let start_x: f32 = 0.0 - WINDOW_HEIGHT * RESOLUTION / 2.0 + TILE_SIZE / 2.0;
    let start_y: f32 = 0.0 + WINDOW_HEIGHT / 2.0 - TILE_SIZE / 2.0;

    let spawn_area_start_x = start_x + TILE_SIZE * 2.0;
    let spawn_area_start_y = start_y - TILE_SIZE * 2.5;
    let spawn_area_end_x = start_x + 13.0 * TILE_SIZE;
    let spawn_area_end_y = start_y - 6.0 * TILE_SIZE;

    let monster_spawn_controller = MonsterSpawnController {
        game_mode: GameMode::ClassicMode,
        max_avalible_monsters: 4,
        require_monster: 5,
        alive_monsters: 0,
        killed_monsters: 0,
        spawn_area_start_x,
        spawn_area_start_y,
        spawn_area_end_x,
        spawn_area_end_y,
    };

    commands.insert_resource(dungeon);
    commands.insert_resource(player_dungeon_stats);
    commands.insert_resource(classic_mode_data);
    commands.insert_resource(monster_spawn_controller);

    state
        .set(SceneState::InGameClassicMode);
}

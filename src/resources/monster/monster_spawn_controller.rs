use bevy::prelude::*;
use crate::resources::game_mode::GameMode;

#[derive(Resource)]
pub struct MonsterSpawnController {
    pub game_mode: GameMode,
    pub alive_monsters: i8,
    pub max_avalible_monsters: i8,
    pub require_monster: i8,
    pub killed_monsters: i8,
    pub spawn_area_start_x: f32,
    pub spawn_area_start_y: f32,
    pub spawn_area_end_x: f32,
    pub spawn_area_end_y: f32,
}

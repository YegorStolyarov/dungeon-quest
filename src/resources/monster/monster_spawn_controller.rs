use crate::resources::game_mode::GameMode;

pub struct MonsterSpawnController {
    game_mode: GameMode,
    alive_monsters: u8,
    max_avalible_monsters: u8,
    require_monster: u8,
}

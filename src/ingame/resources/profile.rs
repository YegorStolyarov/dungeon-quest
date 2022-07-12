use serde::{Deserialize, Serialize};

use crate::ingame::resources::game_mode::GameMode;
use crate::ingame::resources::hero::{Gender, HeroClass};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub game_mode: GameMode,
    pub hero_class: HeroClass,
    pub gender: Gender,
    pub total_killed_monsters: usize,
    pub total_cleared_rooms: usize,
    pub total_cleared_waves: usize,
    pub date: String,
    pub playtime: u64,
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            name: String::new(),
            game_mode: GameMode::ClassicMode,
            hero_class: HeroClass::Elf,
            gender: Gender::Male,
            total_cleared_rooms: 0,
            total_killed_monsters: 0,
            total_cleared_waves: 0,
            date: String::new(),
            playtime: 0,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }
}

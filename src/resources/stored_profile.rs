use serde::{Deserialize, Serialize};

use crate::resources::game_mode::GameMode;
use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredProfile {
    pub name: String,
    pub game_mode: GameMode,
    pub hero_class: HeroClass,
    pub gender: Gender,
    pub total_killed_monsters: usize,
    pub total_cleared_rooms: usize,
    pub total_cleared_waves: usize,
    pub date: String,
    pub playtime: i64,
}

use serde::{Deserialize, Serialize};

use crate::ingame::resources::power_bonus::PowerBonus;
use crate::ingame::resources::skill_type::SkillType;
use crate::ingame::resources::weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseStats {
    power_bonus: PowerBonus,
    health_points: f32,
    speed: f32,
    strength: f32,
    intelligence: f32,
    critical_chance: f32,
    dodge_chance: f32,
    restore_chance: f32,
    weapon: WeaponType,
    skill: SkillType,
}

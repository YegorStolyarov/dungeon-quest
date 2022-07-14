use serde::{Deserialize, Serialize};

use crate::ingame::resources::effect_type::EffectType;
use crate::ingame::resources::power_bonus::PowerBonus;
use crate::ingame::resources::weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    name: WeaponType,
    strength: f32,
    intelligence: f32,
    power_bonus: PowerBonus,
    effects: Vec<EffectType>,
}

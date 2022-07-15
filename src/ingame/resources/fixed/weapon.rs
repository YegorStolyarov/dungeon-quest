use serde::{Deserialize, Serialize};

use crate::ingame::resources::fixed::trigger_effect::TriggerEffect;
use crate::ingame::resources::fixed::weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: WeaponType,
    pub strength: f32,
    pub intelligence: f32,
    pub trigger_effect: Option<TriggerEffect>,
}

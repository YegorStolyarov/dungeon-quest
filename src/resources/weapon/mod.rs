use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::resources::effect::effect_type::EffectType;

pub mod attack_type;
pub mod bullet;
pub mod weapon_type;

use attack_type::AttackType;
use bullet::Bullet;
use weapon_type::WeaponType;

#[derive(Serialize, Deserialize, Debug, Clone, Inspectable)]
pub struct Weapon {
    pub name: WeaponType,
    pub attack_type: AttackType,
    pub swing_speed: Option<f32>,
    pub strength: f32,
    pub intelligence: f32,
    pub level: u8,
    pub width: f32,
    pub height: f32,
    pub effect: Option<EffectType>,
    pub trigger_chance: Option<f32>,
    pub effect_bonus: Option<f32>,
    pub bullet: Option<Bullet>,
    pub cooldown: Option<u64>,
    pub scale: f32,
}

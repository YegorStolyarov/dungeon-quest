use serde::{Deserialize, Serialize};

pub mod monster_class;
pub mod monster_skill;
pub mod monster_spawn_controller;

use crate::resources::effect::effect_type::EffectType;
use monster_class::MonsterClass;
use monster_skill::MonsterSkill;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monster {
    pub class: MonsterClass,
    pub damage: f32,
    pub level: u8,
    pub speed: f32,
    pub health_points: f32,
    pub trigger_effect: Option<EffectType>,
    pub trigger_chance: Option<f32>,
    pub skill: Option<MonsterSkill>,
    pub origin_width: f32,
    pub origin_height: f32,
}

use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::resources::effect::effect_type::EffectType;
use crate::resources::monster::monster_class::MonsterClass;
use crate::resources::monster::monster_skill::MonsterSkill;

#[derive(Component, InspectorOptions)]
pub struct MonsterComponent {
    pub current_health_points: f32,
    pub max_health_points: f32,
    pub class: MonsterClass,
    pub damage: f32,
    pub level: u8,
    pub speed: f32,
    pub trigger_effect: Option<EffectType>,
    pub trigger_chance: f32,
    pub skill: Option<MonsterSkill>,
    pub width: f32,
    pub height: f32,
}

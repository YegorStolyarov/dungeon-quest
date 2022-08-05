use bevy::prelude::*;

use crate::resources::potion::potion_type::PotionType;

#[derive(Component)]
pub struct PotionComponent {
    pub potion_type: PotionType,
    pub height: f32,
    pub width: f32,
}

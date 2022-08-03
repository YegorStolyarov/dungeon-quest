use bevy::prelude::*;

use crate::resources::weapon::Weapon;

#[derive(Component)]
pub struct WeaponSwingAttackComponent {
    pub attack_duration: Timer,
    pub swing_speed: f32,
    pub is_swinging: bool,
}

impl WeaponSwingAttackComponent {
    pub fn upgrade(&mut self, weapon: &Weapon) {
        self.swing_speed = weapon.swing_speed.unwrap_or(0.0);
    }
}

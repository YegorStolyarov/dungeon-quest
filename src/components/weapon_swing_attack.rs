use crate::resources::weapon::Weapon;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponSwingAttackComponent {
    pub swing_speed: f32,
    pub stop_angle: f32,
    pub is_swinging: bool,
}

impl WeaponSwingAttackComponent {
    pub fn upgrade(&mut self, weapon: &Weapon) {
        self.swing_speed = weapon.swing_speed.unwrap_or(0.0);
    }
}

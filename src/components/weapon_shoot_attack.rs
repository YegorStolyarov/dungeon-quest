use bevy::prelude::*;
use std::time::Duration;

use crate::resources::weapon::bullet::Bullet;
use crate::resources::weapon::Weapon;

#[derive(Component)]
pub struct WeaponShootAttackComponent {
    pub bullet_information: Bullet,
    pub bullet_target_x: f32,
    pub bullet_target_y: f32,
    pub spawn_bullet: bool,
    pub cooldown_second: u64,
    pub cooldown: Timer,
}

impl WeaponShootAttackComponent {
    pub fn upgrade(&mut self, weapon: &Weapon) {
        self.bullet_information = weapon.bullet.unwrap_or(Bullet {
            width: 0.0,
            height: 0.0,
            speed: 0.0,
            scale: 0.0,
        });
        self.bullet_target_x = 0.0;
        self.bullet_target_y = 0.0;
        self.spawn_bullet = false;
        self.cooldown_second = weapon.cooldown.unwrap_or(0);
        self.cooldown = Timer::new(Duration::from_secs(1), false);
    }
}

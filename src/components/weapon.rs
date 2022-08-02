use bevy::prelude::*;
use std::time::Duration;

use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::bullet::Bullet;
use crate::resources::weapon::weapon_type::WeaponType;
use crate::resources::weapon::Weapon;

#[derive(Component)]
pub struct WeaponComponent {
    pub bullet_information: Bullet,
    pub bullet_target_x: f32,
    pub bullet_target_y: f32,
    pub spawn_bullet: bool,

    pub attack_type: AttackType,
    pub cooldown_second: u64,
    pub swing_speed: f32,
    pub cooldown: Timer,

    pub name: WeaponType,
    pub level: u8,

    pub size_height: f32,
    pub size_width: f32,
    pub scale: f32,
}

impl WeaponComponent {
    pub fn upgrade_weapon(&mut self, weapon: Weapon) {
        self.bullet_information = weapon.bullet.unwrap_or(Bullet {
            width: 0.0,
            height: 0.0,
            speed: 0.0,
            scale: 0.0,
        });
        self.bullet_target_x = 0.0;
        self.bullet_target_y = 0.0;
        self.spawn_bullet = false;

        self.attack_type = weapon.attack_type.clone();
        self.cooldown_second = weapon.cooldown.unwrap_or(0);
        self.swing_speed = weapon.swing_speed.unwrap_or(0.0);
        self.cooldown = Timer::new(Duration::from_secs(1), false);

        self.name = weapon.name.clone();
        self.level = weapon.level;

        self.size_width = weapon.width;
        self.size_height = weapon.height;
        self.scale = weapon.scale;
    }
}

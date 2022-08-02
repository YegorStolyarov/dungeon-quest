use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::time::Duration;

use crate::components::weapon::WeaponComponent;
use crate::components::weapon_shoot_attack::WeaponShootAttackComponent;
use crate::components::weapon_swing_attack::WeaponSwingAttackComponent;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::weapon::WeaponEntity;
use crate::resources::game_data::GameData;
use crate::resources::profile::Profile;
use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::bullet::Bullet;
use crate::resources::weapon::weapon_type::WeaponType;

pub fn initiate_weapon(
    ingame_materials: Res<InGameMaterials>,
    game_data: Res<GameData>,
    mut commands: Commands,
    profile: Res<Profile>,
) {
    let class = profile.hero_class.clone();
    let weapon = game_data.get_weapon(class);

    let weapon_width = weapon.width;
    let weapon_height = weapon.height;

    let weapon_texture = match weapon.name {
        WeaponType::Bow => ingame_materials.weapons_materials.bow.clone(),
        WeaponType::ShortSword => ingame_materials.weapons_materials.short_sword.clone(),
        WeaponType::SmallWand => ingame_materials.weapons_materials.small_wand.clone(),
        WeaponType::SmallHammer => ingame_materials.weapons_materials.small_hammer.clone(),
        _ => ingame_materials.weapons_materials.short_sword.clone(),
    };

    let scale = weapon.scale;

    let mut attack_duration = Timer::new(Duration::from_secs(0), false);
    attack_duration.tick(Duration::from_secs(0));

    let mut cooldown = Timer::new(Duration::from_secs(0), false);
    cooldown.tick(Duration::from_secs(0));

    let bullet = weapon.bullet.clone().unwrap_or(Bullet {
        width: 0.0,
        height: 0.0,
        speed: 0.0,
        scale: 0.0,
    });

    let weapon_entity = commands
        .spawn_bundle(SpriteBundle {
            texture: weapon_texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(weapon_width * scale, weapon_height * scale)),
                anchor: match weapon.attack_type {
                    AttackType::Swing => Anchor::BottomCenter,
                    AttackType::Throw => Anchor::BottomCenter,
                    AttackType::Shoot => Anchor::Center,
                },
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Weapon"))
        .insert(WeaponComponent {
            strength: weapon.strength,
            intelligence: weapon.intelligence,
            level: weapon.level,
            name: weapon.name.clone(),
            attack_type: weapon.attack_type.clone(),
            scale,
            size_width: weapon_width,
            size_height: weapon_height,
        })
        .insert(WeaponSwingAttackComponent {
            swing_speed: weapon.swing_speed.unwrap_or(0.0),
            is_swinging: false,
            stop_angle: 0.0,
        })
        .insert(WeaponShootAttackComponent {
            bullet_information: bullet,
            spawn_bullet: false,
            bullet_target_x: 0.0,
            bullet_target_y: 0.0,
            cooldown_second: weapon.cooldown.unwrap_or(0),
            cooldown,
        })
        .id();

    commands.insert_resource(WeaponEntity {
        entity: weapon_entity,
    });
}

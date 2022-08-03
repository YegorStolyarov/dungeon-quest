use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

use crate::components::bullet::BulletComponent;
use crate::components::invinsible_cooldown::InvisibleCooldownComponent;
use crate::components::monster_list_effects::MonsterListEffectsComponent;
use crate::components::weapon_shoot_attack::WeaponShootAttackComponent;
use crate::components::{
    monster::MonsterComponent, player::PlayerComponent, weapon::WeaponComponent,
};
use crate::resources::hero::power::Power;
use crate::resources::weapon::attack_type::AttackType;

pub fn bullet_collision(
    mut commands: Commands,
    player_query: Query<&PlayerComponent>,
    weapon_query: Query<(&WeaponComponent, &WeaponShootAttackComponent)>,
    mut bullets_query: Query<
        (Entity, &Transform),
        (Without<MonsterComponent>, With<BulletComponent>),
    >,
    mut monsters_query: Query<
        (
            &mut MonsterComponent,
            &mut MonsterListEffectsComponent,
            &mut InvisibleCooldownComponent,
            &Transform,
        ),
        (Without<BulletComponent>, With<MonsterComponent>),
    >,
) {
    let (weapon, weapon_shoot_attack) = weapon_query.single();

    if weapon.attack_type == AttackType::Shoot {
        let player = player_query.single();

        let mut damage = if player.power == Power::Intelligence {
            player.intelligence + weapon.intelligence
        } else {
            player.strength + weapon.strength
        };

        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < player.critical_chance {
            damage += 1.0;
        }

        for (bullet_entity, bullet_transform) in bullets_query.iter_mut() {
            let mut bullet_position = bullet_transform.translation;
            bullet_position.z = 0.16;

            let bullet_size = Vec2::new(
                weapon_shoot_attack.bullet_information.width
                    * weapon_shoot_attack.bullet_information.scale,
                weapon_shoot_attack.bullet_information.height
                    * weapon_shoot_attack.bullet_information.scale,
            );

            for (mut monster, mut monster_list_effects, mut invinsible_cooldown, transform) in
                monsters_query.iter_mut()
            {
                let monster_size = Vec2::new(monster.width, monster.height);
                let monster_position = transform.translation;

                if collide(bullet_position, bullet_size, monster_position, monster_size).is_some() {
                    let debuff_effect = weapon.debuff_effect;
                    let trigger_chance = weapon.trigger_chance;

                    if debuff_effect != None && trigger_chance > 0.0 {
                        if rng.gen_range(0.0..1.0) < trigger_chance {
                            monster_list_effects.activate(debuff_effect.unwrap());
                        }
                    }

                    invinsible_cooldown.hurt_duration =
                        Timer::new(Duration::from_secs_f32(0.3), false);

                    monster.current_health_points = if damage > monster.current_health_points {
                        0.0
                    } else {
                        monster.current_health_points - damage
                    };

                    commands.entity(bullet_entity).despawn_recursive();
                    break;
                }
            }
        }
    }
}

pub fn swing_weapon_collision(
    player_query: Query<&PlayerComponent>,
    weapon_query: Query<
        (&WeaponComponent, &Transform),
        (Without<MonsterComponent>, With<WeaponComponent>),
    >,
    mut monsters_query: Query<
        (
            &mut MonsterComponent,
            &mut MonsterListEffectsComponent,
            &mut InvisibleCooldownComponent,
            &Transform,
        ),
        (Without<WeaponComponent>, With<MonsterComponent>),
    >,
) {
    let (weapon, weapon_transform) = weapon_query.single();
    if weapon.attack_type == AttackType::Swing {
        let player = player_query.single();

        let mut weapon_position = weapon_transform.translation;
        weapon_position.z = 0.16;
        let weapon_size = Vec2::new(
            weapon.size_width * weapon.scale,
            weapon.size_height * weapon.scale,
        );

        let mut damage = if player.power == Power::Intelligence {
            player.intelligence + weapon.intelligence
        } else {
            player.strength + weapon.strength
        };

        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < player.critical_chance {
            damage += 1.0;
        }

        for (mut monster, mut monster_list_effects, mut invinsible_cooldown, transform) in
            monsters_query.iter_mut()
        {
            let monster_size = Vec2::new(monster.width, monster.height);
            let monster_position = transform.translation;
            if collide(weapon_position, weapon_size, monster_position, monster_size).is_some() {
                let debuff_effect = weapon.debuff_effect;
                let trigger_chance = weapon.trigger_chance;

                if debuff_effect != None && trigger_chance > 0.0 {
                    if rng.gen_range(0.0..1.0) < trigger_chance {
                        monster_list_effects.activate(debuff_effect.unwrap());
                    }
                }

                invinsible_cooldown.hurt_duration = Timer::new(Duration::from_secs_f32(0.3), false);

                monster.current_health_points = if damage > monster.current_health_points {
                    0.0
                } else {
                    monster.current_health_points - damage
                };
            }
        }
    }
}

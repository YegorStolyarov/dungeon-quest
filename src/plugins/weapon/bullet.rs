// use bevy::math::{Quat, Vec2};
// use bevy::prelude::*;
// use std::f32::consts::PI;
// use std::time::Duration;
//
// use crate::components::bullet::BulletComponent;
// use crate::components::weapon::WeaponComponent;
// use crate::materials::ingame::InGameMaterials;
// use crate::resources::weapon::attack_type::AttackType;
// use crate::resources::weapon::weapon_type::WeaponType;
//
// pub fn spawn_bullet(
//     mut weapon_query: Query<(&mut WeaponComponent, &Transform)>,
//     ingame_materials: Res<InGameMaterials>,
//     mut commands: Commands,
// ) {
//     let (mut weapon_component, weapon_transform) = weapon_query.single_mut();
//
//     if weapon_component.attack_type == AttackType::Shoot {
//         if weapon_component.spawn_bullet {
//             let texture = match weapon_component.name {
//                 WeaponType::Bow => ingame_materials.bullet_materials.arrow.clone(),
//                 _ => ingame_materials.bullet_materials.bullet.clone(),
//             };
//
//             let bullet_information = weapon_component.bullet_information.clone();
//
//             let start_y = weapon_transform.translation.y;
//             let start_x = weapon_transform.translation.x;
//
//             let target_x = weapon_component.bullet_target_x;
//             let target_y = weapon_component.bullet_target_y;
//
//             let delta_x = start_x - target_x;
//             let delta_y = start_y - target_y;
//
//             let angle = delta_y.atan2(delta_x);
//
//             let rotate_z = if angle >= 0.0 && angle <= PI / 2.0 {
//                 -(3.0 * PI / 2.0 - angle)
//             } else if angle > PI / 2.0 && angle <= PI {
//                 -(3.0 * PI / 2.0 - angle)
//             } else if angle >= -PI / 2.0 && angle < 0.0 {
//                 PI / 2.0 - angle.abs()
//             } else {
//                 PI / 2.0 + angle
//             };
//
//             let color = match weapon_component.name {
//                 WeaponType::Bow => Color::default(),
//                 WeaponType::SmallWand => Color::GRAY,
//                 WeaponType::MagicWand => Color::default(),
//                 _ => Color::CYAN,
//             };
//
//             weapon_component.spawn_bullet = false;
//
//             commands
//                 .spawn_bundle(SpriteBundle {
//                     sprite: Sprite {
//                         custom_size: Some(Vec2::new(
//                             bullet_information.width * bullet_information.scale,
//                             bullet_information.height * bullet_information.scale,
//                         )),
//                         color,
//                         ..Default::default()
//                     },
//                     transform: Transform {
//                         translation: Vec3::new(start_x, start_y, 0.2),
//                         rotation: Quat::from_rotation_z(rotate_z),
//                         ..Default::default()
//                     },
//                     texture,
//                     ..Default::default()
//                 })
//                 .insert(Name::new("Bullet"))
//                 .insert(BulletComponent {
//                     target_x: weapon_component.bullet_target_x * 5.0,
//                     target_y: weapon_component.bullet_target_y * 5.0,
//                     duration: Timer::new(Duration::from_secs(3), false),
//                     speed: bullet_information.speed,
//                 });
//         }
//     }
// }
//
// pub fn bullet_handle(
//     mut bullet_query: Query<(Entity, &mut BulletComponent, &mut Transform)>,
//     mut commands: Commands,
//     time: Res<Time>,
// ) {
//     for (entity, mut bullet_component, mut transform) in bullet_query.iter_mut() {
//         if !bullet_component.duration.finished() {
//             bullet_component.duration.tick(time.delta());
//             let target: Vec3 =
//                 Vec3::new(bullet_component.target_x, bullet_component.target_y, 0.02);
//             let dir = (target - transform.translation).normalize();
//             transform.translation += dir * bullet_component.speed;
//
//             if (transform.translation.x - bullet_component.target_x).abs() <= 1.0
//                 && (transform.translation.y - bullet_component.target_y).abs() <= 1.0
//             {
//                 commands.entity(entity).despawn_recursive();
//             }
//         } else {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }

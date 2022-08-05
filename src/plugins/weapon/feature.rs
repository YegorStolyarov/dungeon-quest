use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::sprite::Anchor;
use std::f32::consts::PI;

use crate::components::player::PlayerComponent;
use crate::components::weapon::WeaponComponent;
use crate::components::weapon_shoot_attack::WeaponShootAttackComponent;
use crate::components::weapon_swing_attack::WeaponSwingAttackComponent;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::camera::Orthographic2DCamera;
use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::weapon_type::WeaponType;

pub fn attach_to_player(
    mut weapon_query: Query<(&WeaponComponent, &mut Transform), Without<PlayerComponent>>,
    player_query: Query<&Transform, (Without<WeaponComponent>, With<PlayerComponent>)>,
) {
    let (weapon_component, mut weapon_transform) = weapon_query.single_mut();
    let player_transform = player_query.single();

    let weapon_size_width = weapon_component.size_width;
    let weapon_size_height = weapon_component.size_height;
    let scale = weapon_component.scale;

    weapon_transform.translation.y =
        player_transform.translation.y - weapon_size_height / 2.0 * scale;
    weapon_transform.translation.x =
        player_transform.translation.x - weapon_size_width / 2.0 * scale;
}

pub fn aim(
    q_camera: Query<(&Camera, &GlobalTransform), With<Orthographic2DCamera>>,
    mut weapon_query: Query<(
        &WeaponComponent,
        &mut WeaponShootAttackComponent,
        &mut WeaponSwingAttackComponent,
        &mut Transform,
    )>,
    wnds: Res<Windows>,
    time: Res<Time>,
) {
    let (camera, camera_transform) = q_camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let mouse_pos: Vec2 = world_pos.truncate();

        let (
            weapon_component,
            mut weapon_shoot_attack,
            mut weapon_swing_attack,
            mut weapon_transform,
        ) = weapon_query.single_mut();

        let weapon_position_x = weapon_transform.translation.x;
        let weapon_position_y = weapon_transform.translation.y;

        let delta_x = weapon_position_x - mouse_pos.x;
        let delta_y = weapon_position_y - mouse_pos.y;

        let angle = delta_y.atan2(delta_x);

        let rotate_z = if angle >= 0.0 && angle <= PI / 2.0 {
            -(3.0 * PI / 2.0 - angle)
        } else if angle > PI / 2.0 && angle <= PI {
            -(3.0 * PI / 2.0 - angle)
        } else if angle >= -PI / 2.0 && angle < 0.0 {
            PI / 2.0 - angle.abs()
        } else {
            PI / 2.0 + angle
        };

        match weapon_component.attack_type {
            AttackType::Swing => {
                if weapon_swing_attack.attack_duration.finished() {
                    weapon_transform.rotation = Quat::from_rotation_z(rotate_z);
                } else {
                    weapon_swing_attack.attack_duration.tick(time.delta());
                    let swing_speed = weapon_swing_attack.swing_speed;
                    let swing_time = weapon_swing_attack.attack_duration.elapsed_secs();
                    weapon_transform.rotation =
                        Quat::from_rotation_z(rotate_z - swing_time * swing_speed);
                }
            }
            AttackType::Shoot => {
                if !weapon_shoot_attack.cooldown.finished() {
                    weapon_shoot_attack.cooldown.tick(time.delta());
                }

                if weapon_component.name == WeaponType::Bow {
                    weapon_transform.rotation = Quat::from_rotation_z(rotate_z + PI / 2.0);
                }

                if weapon_component.name == WeaponType::Spear {
                    weapon_transform.rotation = Quat::from_rotation_z(rotate_z);
                }
                
                if weapon_component.name == WeaponType::MagicSword {
                    weapon_transform.rotation = Quat::from_rotation_z(rotate_z);
                }

                weapon_shoot_attack.bullet_target_x = mouse_pos.x;
                weapon_shoot_attack.bullet_target_y = mouse_pos.y;
            }
        }
    }
}

pub fn change_weapon_texture(
    mut weapon_query: Query<(
        &WeaponComponent,
        &mut Sprite,
        &mut Handle<Image>,
        ChangeTrackers<WeaponComponent>,
    )>,
    ingame_materials: Res<InGameMaterials>,
) {
    let (weapon, mut sprite, mut texture, tracker) = weapon_query.single_mut();
    if tracker.is_changed() {
        sprite.custom_size = Some(Vec2::new(
            weapon.size_width * weapon.scale,
            weapon.size_height * weapon.scale,
        ));

        sprite.anchor = match weapon.attack_type {
            AttackType::Swing => Anchor::BottomCenter,
            AttackType::Shoot => Anchor::Center,
        };

        *texture = match weapon.name {
            WeaponType::ShortSword => ingame_materials.weapons_materials.short_sword.clone(),
            WeaponType::Sword => ingame_materials.weapons_materials.sword.clone(),
            WeaponType::BigMachete => ingame_materials.weapons_materials.machete.clone(),
            WeaponType::SmallWand => ingame_materials.weapons_materials.small_wand.clone(),
            WeaponType::MagicWand => ingame_materials.weapons_materials.magic_wand.clone(),
            WeaponType::MagicSword => ingame_materials.weapons_materials.magic_sword.clone(),
            WeaponType::Mace => ingame_materials.weapons_materials.mace.clone(),
            WeaponType::BigHammer => ingame_materials.weapons_materials.big_hammer.clone(),
            WeaponType::SmallHammer => ingame_materials.weapons_materials.small_hammer.clone(),
            WeaponType::Bow => ingame_materials.weapons_materials.bow.clone(),
            WeaponType::Spear => ingame_materials.weapons_materials.spear.clone(),
        };
    }
}

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use std::f32::consts::PI;
use std::time::Duration;

use crate::ingame::resources::player::Player;
use crate::ingame::resources::weapon::attack_type::AttackType;
use crate::ingame::resources::weapon::bullet_controller::BulletController;
use crate::ingame::resources::weapon::weapon_type::WeaponType;
use crate::ingame::weapon::WeaponComponent;
use crate::plugins::camera::Orthographic2DCamera;

pub fn attach_to_player(
    mut weapon_query: Query<(&WeaponComponent, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, (Without<WeaponComponent>, With<Player>)>,
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
    mut weapon_query: Query<(&mut WeaponComponent, &mut Transform)>,
    mut bullet_controller: ResMut<BulletController>,
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

        let (mut weapon_component, mut weapon_transform) = weapon_query.single_mut();

        let weapon_position_x = weapon_transform.translation.x;
        let weapon_position_y = weapon_transform.translation.y;

        let delta_x = weapon_position_x - mouse_pos.x;
        let delta_y = weapon_position_y - mouse_pos.y;

        let angle = delta_y.atan2(delta_x);

        if !weapon_component.cooldown.finished() {
            weapon_component.cooldown.tick(time.delta());
        }

        match weapon_component.attack_type {
            AttackType::Swing => {
                let swing_speed = weapon_component.swing_speed;

                if weapon_component.attack_duration.finished() {
                    weapon_transform.rotation = Quat::from_rotation_z(angle + PI * 3.0 / 4.0);
                } else {
                    weapon_component.attack_duration.tick(time.delta());
                    weapon_transform.rotation = Quat::from_rotation_z(
                        angle + PI * 3.0 / 4.0
                            - weapon_component.attack_duration.elapsed_secs() * swing_speed,
                    );
                }
            }
            AttackType::Shoot => {
                if weapon_component.name == WeaponType::Bow {
                    bullet_controller.target_x = mouse_pos.x * -1.0;
                    bullet_controller.target_y = mouse_pos.y * -1.0;
                    weapon_transform.rotation = Quat::from_rotation_z(angle);
                } else {
                    bullet_controller.target_x = mouse_pos.x;
                    bullet_controller.target_y = mouse_pos.y;
                }
            }
            AttackType::Throw => {}
        }
    }
}

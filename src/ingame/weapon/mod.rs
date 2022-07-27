use bevy::prelude::*;

use crate::scenes::SceneState;

mod bullet;
mod cleanup;
mod feature;
mod initiate;

use crate::ingame::resources::weapon::attack_type::AttackType;
use crate::ingame::resources::weapon::weapon_type::WeaponType;

#[derive(Component)]
pub struct WeaponComponent {
    pub name: WeaponType,
    pub attack_type: AttackType,
    pub attack_duration: Timer,
    pub swing_speed: f32,
    pub cooldown: Timer,
    pub scale: f32,
    pub size_width: f32,
    pub size_height: f32,
    pub cooldown_second: u64,
    pub level: u8,
}

pub struct WeaponPlugin;

pub struct WeaponEntity {
    pub entity: Entity,
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PreClassicMode).with_system(initiate::initiate_weapon),
        );

        app.add_system_set(SystemSet::on_enter(SceneState::InGameClassicMode));

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(feature::attach_to_player),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicMode).with_system(cleanup::cleanup_weapon),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::PreSurvivalMode).with_system(initiate::initiate_weapon),
        );

        app.add_system_set(SystemSet::on_enter(SceneState::InGameSurvivalMode));

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(feature::attach_to_player)
                .with_system(feature::aim)
                .with_system(bullet::spawn_bullet)
                .with_system(bullet::bullet_handle),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode).with_system(cleanup::cleanup_weapon),
        );
    }
}
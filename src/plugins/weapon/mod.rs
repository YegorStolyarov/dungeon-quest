use bevy::prelude::*;

use crate::scenes::SceneState;

mod bullet;
mod cleanup;
mod feature;
mod initiate;

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

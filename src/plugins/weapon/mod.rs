use bevy::prelude::*;

use crate::scenes::SceneState;

mod bullet;
mod cleanup;
mod collisions;
mod feature;
mod initiate;

pub struct WeaponPlugin;

#[derive(Resource)]
pub struct WeaponEntity {
    pub entity: Entity,
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::PreClassicMode),initiate::initiate_weapon);
        app.add_systems(OnEnter(SceneState::PreSurvivalMode),initiate::initiate_weapon);

        app.add_systems(Update, (
            feature::attach_to_player,
            feature::aim,
            feature::change_weapon_texture,
            bullet::spawn_bullet,
            bullet::bullet_handle,
            collisions::bullet_collision,
            collisions::swing_weapon_collision
        ).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

        app.add_systems(OnExit(SceneState::InGameClassicMode),(
            cleanup::cleanup_weapon,
            cleanup::cleanup_bullet
        ));
        app.add_systems(OnExit(SceneState::InGameSurvivalMode),(
            cleanup::cleanup_weapon,
            cleanup::cleanup_bullet
        ));
    }
}

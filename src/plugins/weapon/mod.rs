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
        app.add_system(initiate::initiate_weapon.in_schedule(OnEnter(SceneState::PreClassicMode)));

        app.add_system(feature::attach_to_player.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(feature::aim.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(feature::change_weapon_texture.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(bullet::spawn_bullet.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(bullet::bullet_handle.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(collisions::bullet_collision.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(collisions::swing_weapon_collision.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

        app.add_system(cleanup::cleanup_weapon.in_schedule(OnExit(SceneState::InGameClassicMode)));
        app.add_system(cleanup::cleanup_bullet.in_schedule(OnExit(SceneState::InGameClassicMode)));

        app.add_system(initiate::initiate_weapon.in_schedule(OnEnter(SceneState::PreSurvivalMode)));

        app.add_system(cleanup::cleanup_weapon.in_schedule(OnExit(SceneState::InGameSurvivalMode)));
        app.add_system(cleanup::cleanup_bullet.in_schedule(OnExit(SceneState::InGameSurvivalMode)));
    }
}

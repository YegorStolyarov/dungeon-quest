use bevy::prelude::*;
use crate::scenes::pause_scene::PauseSceneData;

use crate::scenes::SceneState;

mod animation;
mod cleanup;
pub mod collisions;
mod effect;
mod health;
mod initiate;
mod invisible;
mod profile;
mod skill;
pub mod stats;
mod ui;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

pub const PLAYER_SIZE_WIDTH: f32 = 16.0 * 3.5;
pub const PLAYER_SIZE_HEIGHT: f32 = 28.0 * 3.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::PreClassicMode), initiate::initiate_player);
        app.add_systems(OnEnter(SceneState::PreSurvivalMode), initiate::initiate_player);

        app.add_systems(OnEnter(SceneState::InGameClassicMode), ui::setup);
        app.add_systems(OnEnter(SceneState::InGameSurvivalMode), ui::setup);

        app.add_systems(Update, (
            invisible::invincible_cooldown,
            invisible::hurt_duration_color,
            effect::update_effects,
            stats::update_stats.after(effect::update_effects),
            collisions::potions_collision,
            ui::hearts_handle,
            ui::skill_duration_handle,
            ui::skill_cooldown_handle,
            ui::information_texts_handle.after(stats::update_stats),
            health::end_run_check,
            profile::finish_run,
            skill::cooldown,
            skill::duration,
            skill::knight_skill
        ).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode)).and_then(not(resource_exists::<PauseSceneData>()))));

        app.add_systems(Update, (
            animation::player_animation_system,
            collisions::monsters_collision_check
        ).run_if(in_state(SceneState::InGameClassicMode)));

        app.add_systems(Update, (
            collisions::monsters_collision_check_survival,
            animation::player_animation_system.after(collisions::monsters_collision_check_survival)
        ).run_if(in_state(SceneState::InGameSurvivalMode)));

        app.add_systems(OnExit(SceneState::InGameClassicMode), (
            cleanup::cleanup_player,
            ui::cleanup
        ));
        app.add_systems(OnExit(SceneState::InGameSurvivalMode), (
            cleanup::cleanup_player,
            ui::cleanup,
            cleanup::save_cleared_waves
        ));
    }
}

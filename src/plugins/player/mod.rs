use bevy::prelude::*;

use crate::scenes::SceneState;

mod animation;
mod cleanup;
pub mod collisions;
mod effect;
mod health;
mod initiate;
mod profile;
mod skill;
mod stats;
mod ui;

pub struct PlayerPlugin;

pub struct PlayerEntity {
    pub entity: Entity,
}

pub const PLAYER_SIZE_WIDTH: f32 = 16.0 * 3.5;
pub const PLAYER_SIZE_HEIGHT: f32 = 28.0 * 3.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PreClassicMode).with_system(initiate::initiate_player),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameClassicMode).with_system(ui::setup),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(effect::update_effects.label("Effect"))
                .with_system(stats::update_stats.label("Stats").after("Effect"))
                .with_system(ui::information_texts_handle.after("Stats"))
                .with_system(ui::hearts_handle)
                .with_system(ui::skill_duration_handle)
                .with_system(ui::skill_cooldown_handle)
                .with_system(animation::player_animation_system)
                .with_system(health::end_run_check)
                .with_system(profile::finish_run)
                .with_system(skill::cooldown)
                .with_system(skill::duration)
                .with_system(skill::knight_skill),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameClassicMode)
                .with_system(cleanup::cleanup_player)
                .with_system(ui::cleanup),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::PreSurvivalMode).with_system(initiate::initiate_player),
        );

        app.add_system_set(
            SystemSet::on_enter(SceneState::InGameSurvivalMode).with_system(ui::setup),
        );

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(effect::update_effects.label("Effect"))
                .with_system(stats::update_stats.label("Stats").after("Effect"))
                .with_system(ui::information_texts_handle.after("Stats"))
                .with_system(ui::hearts_handle)
                .with_system(ui::skill_duration_handle)
                .with_system(ui::skill_cooldown_handle)
                .with_system(animation::player_animation_system)
                .with_system(health::end_run_check)
                .with_system(profile::finish_run)
                .with_system(skill::cooldown)
                .with_system(skill::duration)
                .with_system(skill::knight_skill),
        );

        app.add_system_set(
            SystemSet::on_exit(SceneState::InGameSurvivalMode)
                .with_system(cleanup::cleanup_player)
                .with_system(ui::cleanup),
        );
    }
}

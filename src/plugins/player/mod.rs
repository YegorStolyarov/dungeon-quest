use bevy::prelude::*;

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
        app.add_system(initiate::initiate_player.in_schedule(OnEnter(SceneState::PreClassicMode)));

        app.add_system(ui::setup.in_schedule(OnEnter(SceneState::InGameClassicMode)));
        app.add_system(invisible::invincible_cooldown.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(invisible::hurt_duration_color.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(effect::update_effects.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(ui::hearts_handle.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(ui::skill_duration_handle.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(collisions::potions_collision.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(ui::skill_cooldown_handle.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(health::end_run_check.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(profile::finish_run.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(skill::cooldown.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(skill::duration.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(skill::knight_skill.run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));

        app.add_system(animation::player_animation_system.in_set(OnUpdate(SceneState::InGameClassicMode)));
        app.add_system(collisions::monsters_collision_check.in_set(OnUpdate(SceneState::InGameClassicMode)));

        app.add_systems((
            cleanup::cleanup_player,
            ui::cleanup
        ).in_schedule(OnExit(SceneState::InGameClassicMode)));

        app.add_system(initiate::initiate_player.in_schedule(OnEnter(SceneState::PreSurvivalMode)));

        app.add_system(ui::setup.in_schedule(OnEnter(SceneState::InGameSurvivalMode)));

        app.add_system(collisions::monsters_collision_check_survival.in_set(OnUpdate(SceneState::InGameSurvivalMode)));

        app.add_system(stats::update_stats.after(effect::update_effects).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(ui::information_texts_handle.after(stats::update_stats).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));
        app.add_system(animation::player_animation_system.after(collisions::monsters_collision_check_survival).run_if(in_state(SceneState::InGameClassicMode).or_else(in_state(SceneState::InGameSurvivalMode))));


        app.add_systems((
            cleanup::cleanup_player,
            ui::cleanup,
            cleanup::save_cleared_waves
        ).in_schedule(OnExit(SceneState::InGameSurvivalMode)));
    }
}

use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::AudioPlugin;

use config::*;

mod config;
mod ingame;
mod materials;
mod plugins;
mod resources;
mod scenes;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT * RESOLUTION,
            height: WINDOW_HEIGHT,
            title: TITLE.to_string(),
            position: Some(Vec2::new(MONITOR_WIDTH / 4.0, MONITOR_HEIGHT / 4.0)),
            resizable: false,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: WINDOW_HEIGHT * RESOLUTION,
                max_width: WINDOW_HEIGHT * RESOLUTION,
                min_height: WINDOW_HEIGHT,
                max_height: WINDOW_HEIGHT,
            },
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .add_state(scenes::SceneState::LoadingScene)
        .add_startup_system(plugins::music::background_audio_channel_setup)
        .add_system(plugins::music::play_background_music)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(scenes::loading_scene::LoadingScenePlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_plugin(scenes::highscore_scene::HighscoreScenePlugin)
        .add_plugin(scenes::options_scene::OptionsScenePlugin)
        .add_plugin(scenes::help_scene::HelpScenePlugin)
        .add_plugin(scenes::credits_scene::CreditsScenePlugin)
        .add_plugin(scenes::game_mode_select_scene::GameModeSelectScenePlugin)
        .add_plugin(scenes::hero_select_scene::HeroSelectScenePlugin)
        .add_plugin(scenes::result_scene::ResultScenePlugin)
        .add_plugin(scenes::pause_scene::PauseScenePlugin)
        .add_plugin(scenes::rewards_scene::RewardsScenePlugin)
        .add_plugin(scenes::reward_scene::RewardScenePlugin)
        .add_plugin(ingame::input::InputHandlePlugin)
        .add_plugin(ingame::player::PlayerPlugin)
        .add_plugin(ingame::weapon::WeaponPlugin)
        .add_plugin(ingame::classic_mode::ClassicModePlugin)
        .add_plugin(ingame::classic_mode::ui::ClassicModeUIPlugin)
        .add_plugin(ingame::survival_mode::SurvivalModePlugin)
        .add_plugin(ingame::survival_mode::ui::SurvivalModeUIPlugin)
        .add_plugin(plugins::debug::DebugPlugin)
        .run();
}

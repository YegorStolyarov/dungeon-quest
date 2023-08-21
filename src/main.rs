use bevy::{prelude::*, window::WindowMode};
use bevy::window::{WindowResolution, WindowResizeConstraints};
use bevy_kira_audio::{AudioPlugin};

use config::*;

mod components;
mod config;
mod materials;
mod plugins;
mod resources;
mod scenes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_HEIGHT * RESOLUTION, WINDOW_HEIGHT),
                title: TITLE.to_string(),
                position: WindowPosition::At(IVec2::new(MONITOR_WIDTH / 4, MONITOR_HEIGHT / 4)),
                resizable: false,
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_HEIGHT * RESOLUTION,
                    max_width: WINDOW_HEIGHT * RESOLUTION,
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                },
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }).set(
            ImagePlugin::default_nearest()
        ))
        .init_resource::<resources::setting::Setting>()
        .init_resource::<resources::dictionary::Dictionary>()
        .add_state::<scenes::SceneState>()

        .add_plugins(AudioPlugin)
        .add_systems(Startup, plugins::music::background_audio_channel_setup)
        .add_systems(Update, plugins::music::play_background_music)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(scenes::loading_scene::LoadingScenePlugin)
        .add_plugins(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_plugins(scenes::highscore_scene::HighscoreScenePlugin)
        .add_plugins(scenes::options_scene::OptionsScenePlugin)
        .add_plugins(scenes::help_scene::HelpScenePlugin)
        .add_plugins(scenes::credits_scene::CreditsScenePlugin)
        .add_plugins(scenes::game_mode_select_scene::GameModeSelectScenePlugin)
        .add_plugins(scenes::hero_select_scene::HeroSelectScenePlugin)
        .add_plugins(scenes::result_scene::ResultScenePlugin)
        .add_plugins(plugins::input::InputHandlePlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_plugins(plugins::weapon::WeaponPlugin)
        .add_plugins(plugins::classic_mode::ClassicModePlugin)
        .add_plugins(plugins::classic_mode::ui::ClassicModeUIPlugin)
        .add_plugins(plugins::survival_mode::SurvivalModePlugin)
        .add_plugins(plugins::survival_mode::ui::SurvivalModeUIPlugin)
        .add_plugins(plugins::monster::MonsterPlugin)
        // .add_plugins(plugins::debug::DebugPlugin)
        .run();
}

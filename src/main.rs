use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::AudioPlugin;

use config::*;

mod config;
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
        .init_resource::<scenes::ApplicationSceneController>()
        .add_state(scenes::ApplicationScene::MainMenuScene)
        .add_startup_system(plugins::music::background_audio_channel_setup)
        .add_system(plugins::music::play_background_music)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        .add_plugin(scenes::setting_scene::SettingScenePlugin)
        // .add_plugin(scenes::loading_scene::LoadingScenePlugin)
        .add_plugin(plugins::debug::DebugPlugin)
        .run();
}

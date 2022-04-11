use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::AudioPlugin;

use config::*;
use state::*;

mod config;
mod plugins;
mod state;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT * RESOLUTION,
            height: WINDOW_HEIGHT,
            title: TITLE.to_string(),
            position: Some(Vec2::new(MONITOR_WIDTH / 4.0, MONITOR_HEIGHT / 4.0)),
            vsync: true,
            resizable: false,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: WINDOW_HEIGHT * RESOLUTION,
                max_width: WINDOW_HEIGHT * RESOLUTION,
                min_height: WINDOW_HEIGHT,
                max_height: WINDOW_HEIGHT,
                ..Default::default()
            },
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<plugins::setting::Setting>()
        .add_state(ApplicationState::MainMenu)
        .add_startup_system(plugins::music::start_background_music)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(plugins::menu::main_menu::MainMenuPlugin)
        .add_plugin(plugins::menu::demos_menu::DemosMenuPlugin)
        .add_plugin(plugins::menu::setting_menu::SettingMenuPlugin)
        .run();
}

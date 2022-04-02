use bevy::{prelude::*, window::WindowMode};
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
        .add_state(ApplicationState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::main_menu::MainMenuPlugin)
        .add_plugin(plugins::demos_menu::DemosMenuPlugin)
        .run();
}

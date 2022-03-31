use bevy::{prelude::*, window::WindowMode};

mod plugins;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 1.0;
pub const HEIGHT: f32 = 540.0;

pub const MONITOR_WIDTH: f32 = 1920.0;
pub const MONITOR_HEIGHT: f32 = 1080.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION * 10.0,
            height: HEIGHT,
            title: "Shoot 'em up".to_string(),
            position: Some(Vec2::new(MONITOR_WIDTH / 4.0, MONITOR_HEIGHT / 4.0)),
            vsync: true,
            resizable: false,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: HEIGHT * RESOLUTION,
                max_width: HEIGHT * RESOLUTION,
                min_height: HEIGHT,
                max_height: HEIGHT,
                ..Default::default()
            },
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, plugins::assets::load_ascii)
        .add_plugin(plugins::main_menu::MainMenuPlugin)
        .run();
}

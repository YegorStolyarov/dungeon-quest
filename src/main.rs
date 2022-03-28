use bevy::prelude::*;

mod plugins;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(window_desscriptor());
    app.add_plugins(DefaultPlugins);
    app.add_plugin(plugins::main_menu::MainMenuPlugin);
    app.run();
}

fn window_desscriptor() -> WindowDescriptor {
    return WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: "Shoot 'Em Up".to_string(),
        vsync: true,
        resizable: false,
        mode: bevy::window::WindowMode::Windowed,
        resize_constraints: bevy::window::WindowResizeConstraints {
            min_width: WINDOW_WIDTH,
            max_width: WINDOW_WIDTH,
            min_height: WINDOW_HEIGHT,
            max_height: WINDOW_HEIGHT,
        },
        ..Default::default()
    };
}

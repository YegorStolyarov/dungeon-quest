use bevy::prelude::*;

const WINDOW_SIDE: f32 = 400.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(window_desscriptor());
    app.add_plugins(DefaultPlugins);
    app.run();
}

fn window_desscriptor() -> WindowDescriptor {
    return WindowDescriptor {
        width: WINDOW_SIDE,
        height: WINDOW_SIDE,
        title: "Blocks".to_string(),
        vsync: true,
        resizable: false,
        mode: bevy::window::WindowMode::Windowed,
        resize_constraints: bevy::window::WindowResizeConstraints {
            min_width: WINDOW_SIDE,
            max_width: WINDOW_SIDE,
            min_height: WINDOW_SIDE,
            max_height: WINDOW_SIDE,
        },
        ..Default::default()
    };
}

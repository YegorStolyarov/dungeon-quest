use bevy::prelude::*;

const BUTTON_WIDTH: f32 = 175.0;
const BUTTON_HEIGHT: f32 = 75.0;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<GameExitEvent>();
        // app.add_event::<StartTetrisEvent>();
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(asset_server.load("images/background.png")),
            ..Default::default()
        })
        .with_children(|parent| {
            let spreate: f32 = BUTTON_HEIGHT / 4.0;

            // Position from right

            // Play Button
            let play_button_position = Vec2::new(10.0, spreate);
            parent.spawn_bundle(new_button(play_button_position, &asset_server));

            // Demos Button
            let demos_button_position = Vec2::new(10.0, spreate * 2.0 + BUTTON_HEIGHT);
            parent.spawn_bundle(new_button(demos_button_position, &asset_server));

            // Setting Button
            let setting_button_position = Vec2::new(10.0, spreate * 3.0 + BUTTON_HEIGHT * 2.0);
            parent.spawn_bundle(new_button(setting_button_position, &asset_server));

            // Quit Button
            let quit_button_position = Vec2::new(10.0, spreate * 4.0 + BUTTON_HEIGHT * 3.0);
            parent.spawn_bundle(new_button(quit_button_position, &asset_server));
        });
}

// position from right side
fn new_button(position: Vec2, asset_server: &Res<AssetServer>) -> ButtonBundle {
    let button_size = Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT));

    ButtonBundle {
        style: Style {
            size: button_size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                right: Val::Px(position.x),
                top: Val::Px(position.y),
                bottom: Val::Auto,
                left: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load("images/button.png")),
        ..Default::default()
    }
}

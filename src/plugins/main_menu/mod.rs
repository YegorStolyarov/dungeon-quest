use bevy::app::AppExit;
use bevy::prelude::*;

use crate::config::*;
use crate::state::*;

#[derive(Component)]
enum MainMenuButton {
    Play,
    Demos,
    Setting,
    Quit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(ApplicationState::MainMenu)
                .with_system(button_system)
                .with_system(setup),
        );
        // .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup.system()));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    commands.spawn_bundle(camera);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(asset_server.load("images/menu_background.png")),
            ..Default::default()
        })
        .with_children(|parent| {
            // Play Button
            let play_button_position = Vec2::new(SEPARATE, SEPARATE);
            parent
                .spawn_bundle(new_button(play_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(new_text("PLAY", &asset_server));
                });

            // Demos Button
            let demos_button_position = Vec2::new(SEPARATE, SEPARATE * 2.0 + BUTTON_HEIGHT);
            parent
                .spawn_bundle(new_button(demos_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(new_text("DEMOS", &asset_server));
                });

            // Setting Button
            let setting_button_position = Vec2::new(SEPARATE, SEPARATE * 3.0 + BUTTON_HEIGHT * 2.0);
            parent
                .spawn_bundle(new_button(setting_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(new_text("SETTING", &asset_server));
                });

            // Quit Button
            let quit_button_position = Vec2::new(SEPARATE, SEPARATE * 4.0 + BUTTON_HEIGHT * 3.0);
            parent
                .spawn_bundle(new_button(quit_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(new_text("QUIT", &asset_server));
                })
                .insert(MainMenuButton::Quit);
        });
}

fn new_button(position: Vec2, asset_server: &Res<AssetServer>) -> ButtonBundle {
    let button_size = Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT));
    ButtonBundle {
        style: Style {
            size: button_size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load("images/EmptyButton.png")),
        ..Default::default()
    }
}

fn new_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/Haedus.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE.into();
                *color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::GREEN.into();
                *color = Color::GREEN.into();
            }
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED.into();
                *color = Color::RED.into();
            }
        }
    }
}

// fn button_system

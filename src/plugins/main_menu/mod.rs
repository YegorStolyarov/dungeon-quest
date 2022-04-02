use bevy::{app::AppExit, prelude::*};

use crate::state::*;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;
const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

static TEXT_FONT: &str = "fonts/Haedus.ttf";

static BUTTON_IMAGE: &str = "images/empty_button.png";
static BACKGROUND_IMAGE: &str = "images/menu_background.png";

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

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
        app.add_system(button_system);
        app.add_system(button_press_system);
        app.add_system_set(SystemSet::on_enter(ApplicationState::MainMenu).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationState::MainMenu).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let play_button_position = Vec2::new(SEPARATE, SEPARATE);
    let demos_button_position = Vec2::new(SEPARATE, SEPARATE * 2.0 + BUTTON_HEIGHT);
    let setting_button_position = Vec2::new(SEPARATE, SEPARATE * 3.0 + BUTTON_HEIGHT * 2.0);
    let quit_button_position = Vec2::new(SEPARATE, SEPARATE * 4.0 + BUTTON_HEIGHT * 3.0);

    let ui_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(asset_server.load(BACKGROUND_IMAGE)),
            ..Default::default()
        })
        .with_children(|parent| {
            // Play Button
            parent
                .spawn_bundle(button_bundle(play_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle("PLAY", &asset_server));
                })
                .insert(MainMenuButton::Play);

            // Demos Button
            parent
                .spawn_bundle(button_bundle(demos_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle("DEMOS", &asset_server));
                })
                .insert(MainMenuButton::Demos);

            // Setting Button
            parent
                .spawn_bundle(button_bundle(setting_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle("SETTING", &asset_server));
                })
                .insert(MainMenuButton::Setting);

            // Quit Button
            parent
                .spawn_bundle(button_bundle(quit_button_position, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle("QUIT", &asset_server));
                })
                .insert(MainMenuButton::Quit);
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

// Text
fn text_bundle(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load(TEXT_FONT),
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

// Button
fn button_bundle(position: Vec2, asset_server: &Res<AssetServer>) -> ButtonBundle {
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
        image: UiImage(asset_server.load(BUTTON_IMAGE)),
        ..Default::default()
    }
}

// Button interaction handle system
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

// Button onClick handle system
fn button_press_system(
    button_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<ApplicationState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MainMenuButton::Demos => state
                    .set(ApplicationState::DemosMenu)
                    .expect("Couldn't switch state to DemosMenu"),
                _ => exit.send(AppExit),
            }
        }
    }
}

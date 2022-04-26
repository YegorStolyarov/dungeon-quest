use bevy::prelude::*;

use crate::config::*;
use crate::state::*;
use crate::status::*;

const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;

const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

const BIG_FONT_SIZE: f32 = 50.0;
const SMALL_FONT_SIZE: f32 = 30.0;

// Button x position at center of window
const BUTTON_X_POSITION: f32 = (WINDOW_HEIGHT * RESOLUTION / 2.0) - (BUTTON_WIDTH / 2.0);

const BUTTON_POSITIONS: [[f32; 2]; 2] = [
    [SEPARATE - 10.0, SEPARATE],   // ReturnHome
    [BUTTON_X_POSITION, SEPARATE], // Movement
];

#[derive(Component, PartialEq)]
enum DemosMenuButton {
    ReturnHome,
    Movement,
}

struct DemosMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct DemosMenuPlugin;

impl Plugin for DemosMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::DemosMenu).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(ApplicationState::DemosMenu).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(ApplicationState::DemosMenu).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
    let ui_root = commands
        .spawn_bundle(root(&asset_server))
        .with_children(|parent| {
            // Return button
            parent
                .spawn_bundle(button_bundle(DemosMenuButton::ReturnHome, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(DemosMenuButton::ReturnHome, &asset_server));
                })
                .insert(DemosMenuButton::ReturnHome);

            // Movement demo button
            parent
                .spawn_bundle(button_bundle(DemosMenuButton::Movement, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(DemosMenuButton::Movement, &asset_server));
                })
                .insert(DemosMenuButton::Movement);
        })
        .id();

    commands.insert_resource(DemosMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<DemosMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

fn root(asset_server: &Res<AssetServer>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(asset_server.load(MENU_BACKGROUND_IMAGE)),
        ..Default::default()
    }
}

fn button_bundle(
    demos_menu_button: DemosMenuButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = match demos_menu_button {
        DemosMenuButton::ReturnHome => {
            Size::new(Val::Px(BUTTON_WIDTH - 50.0), Val::Px(BUTTON_HEIGHT - 20.0))
        }
        DemosMenuButton::Movement => Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
    };

    let possition: [f32; 2] = match demos_menu_button {
        DemosMenuButton::ReturnHome => BUTTON_POSITIONS[0],
        DemosMenuButton::Movement => BUTTON_POSITIONS[1],
    };

    ButtonBundle {
        style: Style {
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(possition[0]),
                top: Val::Px(possition[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load(SMALL_BUTTON_IMAGE)),
        ..Default::default()
    }
}

// Button handle system
fn button_handle_system(
    mut button_query: Query<
        (&DemosMenuButton, &Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<ApplicationState>>,
    mut application_status: ResMut<ApplicationStatus>,
) {
    for (button, interaction, mut color, children) in button_query.iter_mut() {
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
                match button {
                    DemosMenuButton::Movement => {
                        if !application_status.is_data_loaded() {
                            state
                                .set(ApplicationState::LoadingScreen)
                                .expect("Couldn't switch state to Loading Screen");
                            application_status.set_next_state(ApplicationState::MovementDemo);
                        } else {
                            state
                                .set(ApplicationState::MovementDemo)
                                .expect("Couldn't switch state to Movement Demo");
                        }
                    }
                    DemosMenuButton::ReturnHome => state
                        .set(ApplicationState::MainMenu)
                        .expect("Couldn't switch state to MainMenu"),
                }
            }
        }
    }
}

fn text_bundle(demos_menu_button: DemosMenuButton, asset_server: &Res<AssetServer>) -> TextBundle {
    let text: &str = match demos_menu_button {
        DemosMenuButton::Movement => "Movement",
        _ => "[Return]",
    };

    let font_size: f32 = match demos_menu_button {
        DemosMenuButton::Movement => BIG_FONT_SIZE,
        DemosMenuButton::ReturnHome => SMALL_FONT_SIZE,
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(HAEDUS_FONT),
                font_size,
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

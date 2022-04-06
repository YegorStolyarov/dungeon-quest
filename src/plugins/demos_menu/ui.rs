use bevy::prelude::*;

use crate::config::*;
use crate::state::*;

const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;

const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

const BIG_FONT_SIZE: f32 = 50.0;
const SMALL_FONT_SIZE: f32 = 30.0;

// Button x position at center of window
const BUTTON_X_POSITION: f32 = (WINDOW_HEIGHT * RESOLUTION / 2.0) - (BUTTON_WIDTH / 2.0);

const BUTTON_POSITIONS: [[f32; 2]; 2] = [
    [BUTTON_X_POSITION, SEPARATE], // Movement
    [SEPARATE - 10.0, SEPARATE],   // Home
];

#[derive(Component, PartialEq)]
pub enum DemosMenuButton {
    Movement,
    Home,
}

pub fn root(asset_server: &Res<AssetServer>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(asset_server.load(MENU_BACKGROUND_IMAGE)),
        ..Default::default()
    }
}

pub fn button_bundle(
    demos_menu_button: DemosMenuButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = match demos_menu_button {
        DemosMenuButton::Movement => Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
        DemosMenuButton::Home => {
            Size::new(Val::Px(BUTTON_WIDTH - 50.0), Val::Px(BUTTON_HEIGHT - 20.0))
        }
    };

    let possition: [f32; 2] = match demos_menu_button {
        DemosMenuButton::Movement => BUTTON_POSITIONS[0],
        DemosMenuButton::Home => BUTTON_POSITIONS[1],
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
pub fn button_handle_system(
    mut button_query: Query<
        (&DemosMenuButton, &Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<ApplicationState>>,
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
                    DemosMenuButton::Movement => state
                        .set(ApplicationState::MainMenu)
                        .expect("Couldn't switch state to MainMenu"),
                    DemosMenuButton::Home => state
                        .set(ApplicationState::MainMenu)
                        .expect("Couldn't switch state to MainMenu"),
                }
            }
        }
    }
}

pub fn text_bundle(
    demos_menu_button: DemosMenuButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match demos_menu_button {
        DemosMenuButton::Movement => "Movement",
        _ => "[Return]",
    };

    let font_size: f32 = match demos_menu_button {
        DemosMenuButton::Movement => BIG_FONT_SIZE,
        DemosMenuButton::Home => SMALL_FONT_SIZE,
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

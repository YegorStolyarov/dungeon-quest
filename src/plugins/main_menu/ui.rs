use bevy::{app::AppExit, prelude::*};

use crate::config::*;
use crate::state::*;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;
const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

const BUTTON_POSITIONS: [[f32; 2]; 4] = [
    [SEPARATE, SEPARATE],                             // Play
    [SEPARATE, SEPARATE * 2.0 + BUTTON_HEIGHT],       // Demos
    [SEPARATE, SEPARATE * 3.0 + BUTTON_HEIGHT * 2.0], // Setting
    [SEPARATE, SEPARATE * 4.0 + BUTTON_HEIGHT * 3.0], // Quit
];

const TEXT_SIZE: f32 = 80.0;

#[derive(Component)]
pub enum MainMenuButton {
    Play,
    Demos,
    Setting,
    Quit,
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

// Button
pub fn button_bundle(
    main_menu_button: MainMenuButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT));

    let possition: [f32; 2] = match main_menu_button {
        MainMenuButton::Play => BUTTON_POSITIONS[0],
        MainMenuButton::Demos => BUTTON_POSITIONS[1],
        MainMenuButton::Setting => BUTTON_POSITIONS[2],
        MainMenuButton::Quit => BUTTON_POSITIONS[3],
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
        image: UiImage(asset_server.load(NORMAL_BUTTON_IMAGE)),
        ..Default::default()
    }
}

// Button interaction handle system
pub fn button_interaction_handle_system(
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
pub fn button_on_click_handle_system(
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

// Text
pub fn text_bundle(
    main_menu_button: MainMenuButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match main_menu_button {
        MainMenuButton::Play => "PLAY",
        MainMenuButton::Demos => "DEMOS",
        MainMenuButton::Setting => "SETTING",
        MainMenuButton::Quit => "QUIT",
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(HAEDUS_FONT),
                font_size: TEXT_SIZE,
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

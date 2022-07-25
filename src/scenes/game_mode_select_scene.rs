use bevy::prelude::*;
use std::slice::Iter;

use crate::config::*;
use crate::ingame::resources::{game_mode::GameMode, profile::Profile};
use crate::materials::scenes::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::materials::Materials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIDE: f32 = 50.0;
const FONT_SIZE: f32 = 35.0;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 10.0;
const BOX_HEIGHT_TILES: f32 = 5.0;

const BOX_ARRAY: [[i8; 10]; 5] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(PartialEq, Component, Clone)]
enum GameModeSelectSceneButton {
    Return,
    ClassicMode,
    SurvivalMode,
}

impl GameModeSelectSceneButton {
    pub fn iterator() -> Iter<'static, GameModeSelectSceneButton> {
        static BUTTONS: [GameModeSelectSceneButton; 3] = [
            GameModeSelectSceneButton::Return,
            GameModeSelectSceneButton::ClassicMode,
            GameModeSelectSceneButton::SurvivalMode,
        ];
        BUTTONS.iter()
    }
}

pub struct GameModeSelectScenePlugin;

struct GameModeSelectSceneData {
    user_interface_root: Entity,
}

impl Plugin for GameModeSelectScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::GameModeSelectScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::GameModeSelectScene)
                .with_system(button_handle_system)
                .with_system(return_button_handle),
        );
        app.add_system_set(
            SystemSet::on_exit(SceneState::GameModeSelectScene).with_system(cleanup),
        );
    }
}

fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    scenes_materials: Res<ScenesMaterials>,
    dictionary: Res<Dictionary>,
) {
    // user interface root
    let user_interface_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            select_game_mode_text(parent, &materials, &dictionary);
            buttons(parent, &scenes_materials, &materials, &dictionary);
        })
        .id();

    commands.insert_resource(GameModeSelectSceneData {
        user_interface_root,
    });

    // Insert new Profile
    commands.insert_resource(Profile::new());
}

fn cleanup(mut commands: Commands, game_mode_select_scene_data: Res<GameModeSelectSceneData>) {
    commands
        .entity(game_mode_select_scene_data.user_interface_root)
        .despawn_recursive();
}

fn root(materials: &Materials) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(materials.sub_menu_background.clone()),
        ..Default::default()
    }
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: Rect<Val> = Rect {
                left: Val::Px(start_left + BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(start_top + BOX_TILE_SIZE * row_index as f32),
                bottom: Val::Auto,
                right: Val::Auto,
            };

            let image: Handle<Image> = match value {
                0 => menu_box_materials.top_right.clone(),
                1 => menu_box_materials.top_center.clone(),
                2 => menu_box_materials.top_left.clone(),
                3 => menu_box_materials.mid_right.clone(),
                4 => menu_box_materials.mid_center.clone(),
                5 => menu_box_materials.mid_left.clone(),
                6 => menu_box_materials.bottom_right.clone(),
                7 => menu_box_materials.bottom_center.clone(),
                8 => menu_box_materials.bottom_left.clone(),
                _ => panic!("Unknown resources"),
            };

            root.spawn_bundle(NodeBundle {
                image: UiImage(image),
                style: Style {
                    position_type: PositionType::Absolute,
                    position,
                    size,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn select_game_mode_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let left_position = if dictionary.get_current_language() == Language::VI {
        340.0
    } else {
        300.0
    };

    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(left_position),
                top: Val::Px(190.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            glossary.shared_text.select_game_mode.clone(),
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::BLACK,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn buttons(
    root: &mut ChildBuilder,
    scenes_materials: &ScenesMaterials,
    materials: &Materials,
    dictionary: &Dictionary,
) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    for (index, button) in GameModeSelectSceneButton::iterator().enumerate() {
        match button {
            GameModeSelectSceneButton::Return => {
                let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();
                root.spawn_bundle(ButtonBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                            top: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                            right: Val::Auto,
                            bottom: Val::Auto,
                        },
                        size: Size {
                            width: Val::Px(RETURN_BUTTON_SIDE),
                            height: Val::Px(RETURN_BUTTON_SIDE),
                        },
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    image: UiImage(handle_image),
                    ..Default::default()
                })
                .insert(button.clone());
            }
            _ => {
                root.spawn_bundle(ButtonBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px((WINDOW_HEIGHT * RESOLUTION - 300.0) / 2.0),
                            top: Val::Px(if index == 1 { 270.0 } else { 330.0 }),
                            right: Val::Auto,
                            bottom: Val::Auto,
                        },
                        size: Size {
                            width: Val::Px(300.0),
                            height: Val::Px(FONT_SIZE),
                        },
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            if index == 1 {
                                glossary.shared_text.classic_mode.clone()
                            } else {
                                glossary.shared_text.survival_mode.clone()
                            },
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::GRAY,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(button.clone());
            }
        }
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &GameModeSelectSceneButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut profile: ResMut<Profile>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK.into(),
            Interaction::Clicked => {
                if *button == GameModeSelectSceneButton::ClassicMode {
                    profile.set_game_mode(GameMode::ClassicMode);
                    state
                        .set(SceneState::HeroSelectScene)
                        .expect("Couldn't switch state to Hero Select Scene");
                } else if *button == GameModeSelectSceneButton::SurvivalMode {
                    profile.set_game_mode(GameMode::SurvivalMode);
                    state
                        .set(SceneState::HeroSelectScene)
                        .expect("Couldn't switch state to Hero Select Scene");
                }
            }
        }
    }
}

fn return_button_handle(
    mut button_query: Query<
        (&Interaction, &GameModeSelectSceneButton, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        if *button == GameModeSelectSceneButton::Return {
            match *interaction {
                Interaction::None => {
                    ui_image.0 = scenes_materials.icon_materials.home_icon_normal.clone()
                }
                Interaction::Hovered => {
                    ui_image.0 = scenes_materials.icon_materials.home_icon_hovered.clone()
                }
                Interaction::Clicked => {
                    ui_image.0 = scenes_materials.icon_materials.home_icon_clicked.clone();
                    state
                        .set(SceneState::MainMenuScene)
                        .expect("Couldn't switch state to Main Menu Scene");
                }
            }
        }
    }
}

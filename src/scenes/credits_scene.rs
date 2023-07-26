use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIDE: f32 = 50.0;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 10.0;
const BOX_HEIGHT_TILES: f32 = 9.0;

const BOX_ARRAY: [[i8; 10]; 9] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(Component, PartialEq)]
struct ReturnButtonComponent;

pub struct CreditsScenePlugin;

#[derive(Resource)]
struct CreditsSceneData {
    user_interface_root: Entity,
}

impl Plugin for CreditsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(SceneState::CreditsScene)));
        app.add_system(button_handle_system.in_set(OnUpdate(SceneState::CreditsScene)));
        app.add_system(cleanup.in_schedule(OnExit(SceneState::CreditsScene)));
    }
}

fn setup(
    scenes_materials: Res<ScenesMaterials>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let user_interface_root = commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage::new(scenes_materials.sub_background_image.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            credits_menu_box(parent, &scenes_materials.menu_box_materials);
            credits_text(parent, &font_materials, &dictionary);
            texts(parent, &font_materials, &dictionary);
            return_button_component(parent, &scenes_materials);
        })
        .id();

    commands.insert_resource(CreditsSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, credits_scene_data: Res<CreditsSceneData>) {
    commands
        .entity(credits_scene_data.user_interface_root)
        .despawn_recursive();
}

fn credits_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: UiRect = UiRect {
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

            root.spawn(ImageBundle {
                image: UiImage::new(image),
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

fn return_button_component(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();

    let size = Size {
        width: Val::Px(RETURN_BUTTON_SIDE),
        height: Val::Px(RETURN_BUTTON_SIDE),
    };

    root.spawn(ButtonBundle {
        style: Style {
            position: UiRect {
                left: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                top: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image: UiImage::new(handle_image),
        ..Default::default()
    })
    .insert(ReturnButtonComponent);
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<ReturnButtonComponent>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<NextState<SceneState>>,
) {
    for (interaction, mut ui_image) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                ui_image.texture = scenes_materials.icon_materials.home_icon_normal.clone()
            }
            Interaction::Hovered => {
                ui_image.texture = scenes_materials.icon_materials.home_icon_hovered.clone()
            }
            Interaction::Clicked => {
                ui_image.texture = scenes_materials.icon_materials.home_icon_clicked.clone();
                state
                    .set(SceneState::MainMenuScene);
            }
        }
    }
}

fn credits_text(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    root.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(445.0),
                top: Val::Px(65.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::from_section(
            glossary.main_menu_scene_text.credits,
            TextStyle {
                font,
                font_size: 50.0,
                color: Color::BLACK,
            }
        ).with_alignment(
            TextAlignment::Center
        ),
        ..Default::default()
    });
}

fn texts(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let file = match File::open(CREDITS_FILE) {
        Ok(file) => file,
        Err(err) => panic!("Can't open credits file: {}", err),
    };

    let lines = io::BufReader::new(file).lines();

    for (index, line) in lines.enumerate() {
        let text = line.unwrap();
        root.spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(260.0),
                    top: Val::Px(110.0 + (index as f32) * 24.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::BLACK,
                }
            ).with_alignment(
                TextAlignment::Center
            ),
            ..Default::default()
        });
    }
}

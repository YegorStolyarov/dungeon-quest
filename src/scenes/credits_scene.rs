use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

use crate::config::*;
use crate::resources::dictionary::Dictionary;
use crate::resources::materials::scenes::MenuBoxMaterials;
use crate::resources::materials::scenes::ScenesMaterials;
use crate::resources::materials::Materials;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIDE: f32 = 50.0;

const CREDITS_BOX_TILE_SIZE: f32 = 60.0;
const CREDITS_BOX_WIDTH_TILES: f32 = 10.0;
const CREDITS_BOX_HEIGHT_TILES: f32 = 9.0;

const CREDITS_BOX_ARRAY: [[i8; 10]; 9] = [
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
enum CreditsSceneButton {
    Return,
}

pub struct CreditsScenePlugin;

struct CreditsSceneData {
    user_interface_root: Entity,
}

impl Plugin for CreditsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::CreditsScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::CreditsScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::CreditsScene).with_system(cleanup));
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
            credits_menu_box(parent, &scenes_materials.menu_box_materials);
            credits_text(parent, &materials, &dictionary);
            texts(parent, &materials, &dictionary);
            return_button(parent, &scenes_materials);
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

fn credits_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(CREDITS_BOX_TILE_SIZE),
        height: Val::Px(CREDITS_BOX_TILE_SIZE),
    };

    let start_left =
        (WINDOW_HEIGHT * RESOLUTION - CREDITS_BOX_TILE_SIZE * CREDITS_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - CREDITS_BOX_TILE_SIZE * CREDITS_BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in CREDITS_BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: Rect<Val> = Rect {
                left: Val::Px(start_left + CREDITS_BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(start_top + CREDITS_BOX_TILE_SIZE * row_index as f32),
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

fn return_button(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();

    let size = Size {
        width: Val::Px(RETURN_BUTTON_SIDE),
        height: Val::Px(RETURN_BUTTON_SIDE),
    };

    root.spawn_bundle(ButtonBundle {
        style: Style {
            position: Rect {
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
        image: UiImage(handle_image),
        ..Default::default()
    })
    .insert(CreditsSceneButton::Return);
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &CreditsSceneButton, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            CreditsSceneButton::Return => match *interaction {
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
            },
        }
    }
}

fn credits_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(445.0),
                top: Val::Px(65.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            glossary.main_menu_scene_text.credits.clone(),
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

fn texts(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let file = match File::open(CREDITS_FILE) {
        Ok(file) => file,
        Err(err) => panic!("Can't open credits file: {}", err.to_string()),
    };

    let lines = io::BufReader::new(file).lines();
    let mut index = 0;

    for line in lines {
        let text = line.unwrap();
        root.spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(260.0),
                    top: Val::Px(110.0 + (index as f32) * 24.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
        index += 1;
    }
}

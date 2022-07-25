use bevy::prelude::*;

use crate::config::*;
use crate::materials::scenes::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::materials::Materials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIDE: f32 = 50.0;
const MENU_BOX_TILE_SIZE: f32 = 60.0;

const HELP_BOX_WIDTH_TILES: f32 = 9.0;
const HELP_BOX_HEIGHT_TILES: f32 = 8.0;

const HELP_BOX_ARRAY: [[i8; 9]; 8] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(Component, PartialEq)]
enum HelpSceneButton {
    Return,
}

pub struct HelpScenePlugin;

struct HelpSceneData {
    user_interface_root: Entity,
}

impl Plugin for HelpScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::HelpScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::HelpScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::HelpScene).with_system(cleanup));
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
            help_menu_box(parent, &scenes_materials.menu_box_materials);
            texts(parent, &materials, &dictionary);
            control_texts(parent, &materials, &dictionary);
            return_button(parent, &scenes_materials)
        })
        .id();

    commands.insert_resource(HelpSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, help_scene_data: Res<HelpSceneData>) {
    commands
        .entity(help_scene_data.user_interface_root)
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

fn help_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(MENU_BOX_TILE_SIZE),
        height: Val::Px(MENU_BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - MENU_BOX_TILE_SIZE * HELP_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - MENU_BOX_TILE_SIZE * HELP_BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in HELP_BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: Rect<Val> = Rect {
                left: Val::Px(start_left + MENU_BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(start_top + MENU_BOX_TILE_SIZE * row_index as f32),
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

fn texts(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let position_of_texts: [[f32; 2]; 8] = [
        [465.0, 100.0],
        [300.0, 160.0],
        [300.0, 205.0],
        [300.0, 250.0],
        [300.0, 295.0],
        [300.0, 340.0],
        [300.0, 385.0],
        [300.0, 430.0],
    ];

    for index in 0..position_of_texts.len() {
        let value: String = match index {
            0 => glossary.help_scene_text.help.clone(),
            1 => glossary.help_scene_text.move_up.clone(),
            2 => glossary.help_scene_text.move_down.clone(),
            3 => glossary.help_scene_text.move_left.clone(),
            4 => glossary.help_scene_text.move_right.clone(),
            5 => glossary.help_scene_text.use_skill.clone(),
            6 => glossary.help_scene_text.attack.clone(),
            7 => glossary.help_scene_text.aim.clone(),
            _ => panic!("Unknown text"),
        };

        let font_size: f32 = match index {
            0 => 50.0,
            _ => 30.0,
        };

        let mut position_left = position_of_texts[index][0];
        let position_top = position_of_texts[index][1];

        if index == 0 && dictionary.get_current_language() == Language::VI {
            position_left = 438.0;
        }

        root.spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(position_left),
                    top: Val::Px(position_top),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                value,
                TextStyle {
                    font: font.clone(),
                    font_size,
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
}

fn control_texts(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());

    let position_of_texts: [[f32; 2]; 7] = [
        [645.0, 160.0],
        [650.0, 205.0],
        [650.0, 250.0],
        [650.0, 295.0],
        [620.0, 340.0],
        [620.0, 385.0],
        [620.0, 430.0],
    ];

    for index in 0..position_of_texts.len() {
        let value: &str = match index {
            0 => "W",
            1 => "S",
            2 => "A",
            3 => "D",
            4 => "SPACE",
            5 => "MOUSE 1",
            6 => "MOUSE",
            _ => panic!("Unknown text"),
        };

        root.spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(position_of_texts[index][0]),
                    top: Val::Px(position_of_texts[index][1]),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                value,
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
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
    .insert(HelpSceneButton::Return);
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &HelpSceneButton, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            HelpSceneButton::Return => match *interaction {
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

use bevy::prelude::*;
use std::slice::Iter;

use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIZE: f32 = 50.0;
const NORMAL_BUTTON_SIZE: f32 = 32.0;

const MENU_BOX_TILE_SIZE: f32 = 60.0;

const MENU_BOX_WIDTH_TILES: f32 = 8.0;
const MENU_BOX_HEIGHT_TILES: f32 = 6.0;

const MENU_BOX_ARRAY: [[i8; 8]; 6] = [
    [0, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 8],
];

const SELECTED_FLAG_COLOR: Color = Color::Rgba {
    red: (160.0 / 255.0),
    green: (170.0 / 255.0),
    blue: (170.0 / 255.0),
    alpha: 1.0,
};

const NORMAL_FLAG_COLOR: Color = Color::Rgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

#[derive(Component, Clone)]
enum PairButtonComponent {
    Vietnamese,
    English,
}

impl PairButtonComponent {
    pub fn iterator() -> Iter<'static, PairButtonComponent> {
        [
            PairButtonComponent::Vietnamese,
            PairButtonComponent::English,
        ]
        .iter()
    }
}

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Return,
    EnableSound,
    EnableMusic,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Return,
            ButtonComponent::EnableSound,
            ButtonComponent::EnableMusic,
        ]
        .iter()
    }
}

#[derive(Component, Clone)]
enum TextComponent {
    Options,
    EnableSound,
    EnableMusic,
    Language,
}

impl TextComponent {
    pub fn iterator() -> Iter<'static, TextComponent> {
        [
            TextComponent::Options,
            TextComponent::EnableSound,
            TextComponent::EnableMusic,
            TextComponent::Language,
        ]
        .iter()
    }
}

pub struct OptionsScenePlugin;

struct OptionsSceneData {
    user_interface_root: Entity,
}

impl Plugin for OptionsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::OptionsScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::OptionsScene)
                .with_system(button_handle_system)
                .with_system(pair_button_handle_system)
                .with_system(text_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::OptionsScene).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    setting: Res<Setting>,
    dictionary: Res<Dictionary>,
) {
    // user interface root
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(scenes_materials.sub_background_image.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            texts(parent, &font_materials, &dictionary);
            buttons(parent, &setting, &scenes_materials);
            pair_buttons(parent, &setting, &scenes_materials);
        })
        .id();
    commands.insert_resource(OptionsSceneData {
        user_interface_root,
    });
}

fn cleanup(
    mut commands: Commands,
    options_scene_data: Res<OptionsSceneData>,
    setting: Res<Setting>,
) {
    setting.store();
    commands
        .entity(options_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(MENU_BOX_TILE_SIZE),
        height: Val::Px(MENU_BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - MENU_BOX_TILE_SIZE * MENU_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - MENU_BOX_TILE_SIZE * MENU_BOX_HEIGHT_TILES) / 2.0;
    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (row_index, row) in MENU_BOX_ARRAY.iter().enumerate() {
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

                parent.spawn_bundle(NodeBundle {
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
    })
    .insert(Name::new("MenuBox"));
}

fn texts(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let position_of_texts: [[f32; 2]; 4] = [
        [440.0, 160.0],
        [320.0, 230.0],
        [320.0, 290.0],
        [320.0, 350.0],
    ];

    for (index, prevalue) in TextComponent::iterator().enumerate() {
        let value: String = match index {
            0 => glossary.options_scene_text.options.clone(),
            1 => glossary.options_scene_text.enable_music.clone(),
            2 => glossary.options_scene_text.enable_sound.clone(),
            3 => glossary.options_scene_text.language.clone(),
            _ => panic!("Unknown text"),
        };

        let component_name = match index {
            0 => "OptionsText",
            1 => "EnableMusicText",
            2 => "EnableSoundText",
            3 => "LanguageText",
            _ => "Unknown text",
        };

        let font_size: f32 = match index {
            0 => 50.0,
            _ => 35.0,
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
                    font_size,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(prevalue.clone());
    }
}

fn buttons(root: &mut ChildBuilder, setting: &Setting, scenes_materials: &ScenesMaterials) {
    let positions: [Rect<Val>; 3] = [
        Rect {
            left: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            top: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        Rect {
            left: Val::Px(610.0),
            top: Val::Px(230.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        Rect {
            left: Val::Px(610.0),
            top: Val::Px(290.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
    ];

    for (index, button) in ButtonComponent::iterator().enumerate() {
        let handle_image = match button {
            ButtonComponent::Return => scenes_materials.icon_materials.home_icon_normal.clone(),
            ButtonComponent::EnableSound => {
                if setting.get_enable_sound() == true {
                    scenes_materials.icon_materials.sound_icon_on.clone()
                } else {
                    scenes_materials.icon_materials.sound_icon_off.clone()
                }
            }
            ButtonComponent::EnableMusic => {
                if setting.get_enable_music() == true {
                    scenes_materials.icon_materials.music_icon_on.clone()
                } else {
                    scenes_materials.icon_materials.music_icon_off.clone()
                }
            }
        };

        let component_name = match button {
            ButtonComponent::Return => "Return",
            ButtonComponent::EnableSound => "EnableSound",
            ButtonComponent::EnableMusic => "EnableMusic",
        };

        let size = match button {
            ButtonComponent::Return => Size {
                width: Val::Px(RETURN_BUTTON_SIZE),
                height: Val::Px(RETURN_BUTTON_SIZE),
            },
            _ => Size {
                width: Val::Px(NORMAL_BUTTON_SIZE),
                height: Val::Px(NORMAL_BUTTON_SIZE),
            },
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                position: positions[index],
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image: UiImage(handle_image),
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(button.clone());
    }
}

fn pair_buttons(root: &mut ChildBuilder, setting: &Setting, scenes_materials: &ScenesMaterials) {
    let positions: [Rect<Val>; 2] = [
        Rect {
            left: Val::Px(570.0),
            top: Val::Px(350.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        Rect {
            left: Val::Px(620.0),
            top: Val::Px(350.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
    ];

    for (index, pair_button) in PairButtonComponent::iterator().enumerate() {
        let component_name = match pair_button {
            PairButtonComponent::Vietnamese => "Vietnamese",
            PairButtonComponent::English => "English",
        };

        let handle_image = match pair_button {
            PairButtonComponent::Vietnamese => scenes_materials.flag_materials.vietnam.clone(),
            PairButtonComponent::English => scenes_materials.flag_materials.united_states.clone(),
        };

        let color = match pair_button {
            PairButtonComponent::Vietnamese => match setting.get_language() {
                Language::VI => SELECTED_FLAG_COLOR,
                Language::EN => NORMAL_FLAG_COLOR,
            },
            PairButtonComponent::English => match setting.get_language() {
                Language::VI => NORMAL_FLAG_COLOR,
                Language::EN => SELECTED_FLAG_COLOR,
            },
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                position: positions[index],
                size: Size {
                    width: Val::Px(NORMAL_BUTTON_SIZE),
                    height: Val::Px(NORMAL_BUTTON_SIZE),
                },
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor(color),
            image: UiImage(handle_image),
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(pair_button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    mut setting: ResMut<Setting>,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            ButtonComponent::Return => match *interaction {
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
            ButtonComponent::EnableSound => match *interaction {
                Interaction::None => {
                    if setting.get_enable_sound() == true {
                        ui_image.0 = scenes_materials.icon_materials.sound_icon_on.clone()
                    } else {
                        ui_image.0 = scenes_materials.icon_materials.sound_icon_off.clone()
                    }
                }
                Interaction::Hovered => {
                    ui_image.0 = scenes_materials.icon_materials.sound_icon_hovered.clone()
                }
                Interaction::Clicked => {
                    let enable_sound = setting.get_enable_sound();
                    setting.set_enable_sound(!enable_sound);
                }
            },
            ButtonComponent::EnableMusic => match *interaction {
                Interaction::None => {
                    if setting.get_enable_music() == true {
                        ui_image.0 = scenes_materials.icon_materials.music_icon_on.clone()
                    } else {
                        ui_image.0 = scenes_materials.icon_materials.music_icon_off.clone()
                    }
                }
                Interaction::Hovered => {
                    ui_image.0 = scenes_materials.icon_materials.music_icon_hovered.clone()
                }
                Interaction::Clicked => {
                    let enable_music = setting.get_enable_music();
                    setting.set_enable_music(!enable_music);
                }
            },
        }
    }
}

fn pair_button_handle_system(
    mut button_query: Query<(&Interaction, &PairButtonComponent, &mut UiColor)>,
    mut setting: ResMut<Setting>,
    mut dictionary: ResMut<Dictionary>,
) {
    for (interaction, button, mut ui_color) in button_query.iter_mut() {
        match *button {
            PairButtonComponent::Vietnamese => match *interaction {
                Interaction::None | Interaction::Hovered => match setting.get_language() {
                    Language::VI => ui_color.0 = SELECTED_FLAG_COLOR,
                    Language::EN => ui_color.0 = NORMAL_FLAG_COLOR,
                },
                Interaction::Clicked => {
                    if setting.get_language() != Language::VI {
                        setting.set_language(Language::VI);
                        dictionary.set_current_language(Language::VI);
                    }
                }
            },
            PairButtonComponent::English => match *interaction {
                Interaction::None | Interaction::Hovered => match setting.get_language() {
                    Language::VI => ui_color.0 = NORMAL_FLAG_COLOR,
                    Language::EN => ui_color.0 = SELECTED_FLAG_COLOR,
                },
                Interaction::Clicked => {
                    if setting.get_language() != Language::EN {
                        setting.set_language(Language::EN);
                        dictionary.set_current_language(Language::EN);
                    }
                }
            },
        };
    }
}

fn text_handle_system(
    mut text_query: Query<(&TextComponent, &mut Text)>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    if dictionary.is_changed() {
        for (text_type, mut text) in text_query.iter_mut() {
            text.sections[0].style.font = font.clone();
            match *text_type {
                TextComponent::Options => {
                    text.sections[0].value = glossary.options_scene_text.options.clone();
                }
                TextComponent::EnableSound => {
                    text.sections[0].value = glossary.options_scene_text.enable_sound.clone();
                }
                TextComponent::EnableMusic => {
                    text.sections[0].value = glossary.options_scene_text.enable_music.clone();
                }
                TextComponent::Language => {
                    text.sections[0].value = glossary.options_scene_text.language.clone();
                }
            }
        }
    }
}

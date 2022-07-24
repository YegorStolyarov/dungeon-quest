use bevy::prelude::*;
use chrono::{DateTime, Datelike, Timelike};
use std::fs::File;
use std::io::prelude::*;
use std::slice::Iter;

use crate::config::*;
use crate::ingame::resources::game_mode::GameMode;
use crate::ingame::resources::stored_profile::StoredProfile;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::materials::scenes::MenuBoxMaterials;
use crate::resources::materials::scenes::ScenesMaterials;
use crate::resources::materials::Materials;
use crate::scenes::SceneState;

use crate::ingame::resources::profile::Profile;

const RETURN_BUTTON_SIDE: f32 = 50.0;
const BUTTON_SIDE: f32 = 70.0;

const USER_INPUT_NAME_BORDER_WIDTH: f32 = 500.0;
const USER_INPUT_NAME_BORDER_HEIGHT: f32 = 50.0;

const MENU_BOX_TILE_SIZE: f32 = 60.0;
const MENU_BOX_WIDTH_TILES: f32 = 9.0;
const MENU_BOX_HEIGHT_TILES: f32 = 9.0;

const MENU_BOX_ARRAY: [[i8; 9]; 9] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(Component, Copy, Clone)]
enum ResultSceneButton {
    Return,
    PlayAgain,
    SaveProfile,
}

#[derive(Component, Copy, Clone)]
enum PrefixWord {
    GameMode,
    Date,
    StartTime,
    EndTime,
    Playtime,
    TotalKilledMonsters,
    TotalClearedRooms,
    TotalClearedWaves,
}

impl PrefixWord {
    pub fn iterator() -> Iter<'static, PrefixWord> {
        static PREFIX_WORDS: [PrefixWord; 8] = [
            PrefixWord::GameMode,
            PrefixWord::Date,
            PrefixWord::StartTime,
            PrefixWord::EndTime,
            PrefixWord::Playtime,
            PrefixWord::TotalKilledMonsters,
            PrefixWord::TotalClearedRooms,
            PrefixWord::TotalClearedWaves,
        ];
        PREFIX_WORDS.iter()
    }
}

struct ResultSceneData {
    user_interface_root: Entity,
}

#[derive(Component, Copy, Clone)]
struct UserInputBox;

#[derive(Component, Copy, Clone)]
struct UserInput;

struct UserInputController(bool);

pub struct ResultScenePlugin;

impl Plugin for ResultScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::ResultScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::ResultScene)
                .with_system(button_handle_system)
                .with_system(user_input_visibility_handle)
                .with_system(user_input_handle),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::ResultScene).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    scenes_materials: Res<ScenesMaterials>,
    profile: Res<Profile>,
    dictionary: Res<Dictionary>,
) {
    // user interface root
    let user_interface_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            result_text(parent, &materials, &dictionary);
            texts(parent, &materials, &dictionary, &profile);
            return_button(parent, &scenes_materials);
            save_profile_button(parent, &scenes_materials, profile);
            play_again_button(parent, &scenes_materials);
            user_input_text(parent, &materials, &dictionary);
        })
        .insert(Name::new("UIRoot"))
        .id();

    commands.insert_resource(ResultSceneData {
        user_interface_root,
    });

    commands.insert_resource(UserInputController(false));
}

fn cleanup(mut commands: Commands, result_scene_data: Res<ResultSceneData>) {
    commands
        .entity(result_scene_data.user_interface_root)
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

fn result_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let left_position = if dictionary.get_current_language() == Language::EN {
        450.0
    } else {
        440.0
    };

    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(left_position),
                top: Val::Px(60.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            glossary.result_scene_text.result.clone(),
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
    })
    .insert(Name::new("ResultText"));
}

fn texts(
    root: &mut ChildBuilder,
    materials: &Materials,
    dictionary: &Dictionary,
    profile: &Profile,
) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (index, prefix) in PrefixWord::iterator().enumerate() {
            let top_position = 110.0 + (index as f32) * 40.0;
            let left_position = 300.0;

            let value: String = match prefix {
                PrefixWord::GameMode => {
                    if profile.game_mode == GameMode::ClassicMode {
                        glossary.shared_text.classic_mode.clone()
                    } else {
                        glossary.shared_text.survival_mode.clone()
                    }
                }
                PrefixWord::Date => {
                    let prefix = glossary.result_scene_text.date.clone();
                    let start_time = profile.start_time.clone();

                    let date = DateTime::parse_from_rfc3339(start_time.as_str())
                        .expect("Error convert time");

                    let year = date.year();

                    let day = date.day();
                    let formated_day = match day {
                        0..=9 => format!("0{}", day),
                        _ => format!("{}", day),
                    };

                    let month = date.month();
                    let formated_month = match month {
                        0..=9 => format!("0{}", month),
                        _ => format!("{}", month),
                    };

                    let value = match dictionary.get_current_language() {
                        Language::VI => format!("{}-{}-{}", formated_day, formated_month, year),
                        Language::EN => format!("{}-{}-{}", formated_month, formated_day, year),
                    };
                    prefix + value.as_str()
                }
                PrefixWord::StartTime => {
                    let prefix = glossary.result_scene_text.start_time.clone();
                    let start_time = profile.start_time.clone();

                    let date = DateTime::parse_from_rfc3339(start_time.as_str())
                        .expect("Error convert time");

                    let hour = date.hour();
                    let formated_hour = match hour {
                        0..=9 => format!("0{}", hour),
                        _ => format!("{}", hour),
                    };
                    let minute = date.minute();
                    let formated_minute = match minute {
                        0..=9 => format!("0{}", minute),
                        _ => format!("{}", minute),
                    };
                    let second = date.second();
                    let formated_second = match second {
                        0..=9 => format!("0{}", second),
                        _ => format!("{}", second),
                    };

                    let format_start_time =
                        format!("{}:{}:{}", formated_hour, formated_minute, formated_second);

                    prefix + format_start_time.as_str()
                }
                PrefixWord::EndTime => {
                    let prefix = glossary.result_scene_text.end_time.clone();
                    let start_time = profile.end_time.clone();

                    let date = DateTime::parse_from_rfc3339(start_time.as_str())
                        .expect("Error convert time");

                    let hour = date.hour();
                    let formated_hour = match hour {
                        0..=9 => format!("0{}", hour),
                        _ => format!("{}", hour),
                    };
                    let minute = date.minute();
                    let formated_minute = match minute {
                        0..=9 => format!("0{}", minute),
                        _ => format!("{}", minute),
                    };
                    let second = date.second();
                    let formated_second = match second {
                        0..=9 => format!("0{}", second),
                        _ => format!("{}", second),
                    };

                    let format_start_time =
                        format!("{}:{}:{}", formated_hour, formated_minute, formated_second);

                    prefix + format_start_time.as_str()
                }
                PrefixWord::TotalKilledMonsters => {
                    let prefix = glossary.result_scene_text.total_killed_monsters.clone();
                    let total_killed_monsters = profile.total_killed_monsters;

                    prefix + total_killed_monsters.to_string().as_str()
                }

                PrefixWord::TotalClearedRooms => {
                    let prefix = glossary.result_scene_text.total_cleared_rooms.clone();
                    let total_cleared_rooms = profile.total_cleared_rooms;

                    prefix + total_cleared_rooms.to_string().as_str()
                }
                PrefixWord::TotalClearedWaves => {
                    let prefix = glossary.result_scene_text.total_cleared_waves.clone();
                    let total_cleared_waves = profile.total_cleared_waves;

                    prefix + total_cleared_waves.to_string().as_str()
                }
                PrefixWord::Playtime => {
                    let prefix = glossary.result_scene_text.playtime.clone();

                    let start_time =
                        DateTime::parse_from_rfc3339(profile.start_time.clone().as_str())
                            .expect("Error convert time");

                    let end_time = DateTime::parse_from_rfc3339(profile.end_time.clone().as_str())
                        .expect("Error convert time");

                    let diff_time = end_time - start_time;

                    let diff_seconds = diff_time.num_seconds();

                    let seconds = diff_seconds % 60;
                    let formated_seconds = match seconds {
                        0..=9 => format!("0{}", seconds),
                        _ => format!("{}", seconds),
                    };

                    let minutes = (diff_seconds / 60) % 60;
                    let formated_minutes = match minutes {
                        0..=9 => format!("0{}", minutes),
                        _ => format!("{}", minutes),
                    };

                    let hours = (diff_seconds / 60) / 60;
                    let formated_hours = match hours {
                        0..=9 => format!("0{}", hours),
                        _ => format!("{}", hours),
                    };

                    let value = format!(
                        "{}:{}:{}",
                        formated_hours, formated_minutes, formated_seconds
                    );
                    prefix + value.as_str()
                }
            };

            let component_name = match prefix {
                PrefixWord::GameMode => "GameMode",
                PrefixWord::Date => "Date",
                PrefixWord::StartTime => "StartTime",
                PrefixWord::EndTime => "EndTime",
                PrefixWord::TotalKilledMonsters => "TotalKilledMonsters",
                PrefixWord::TotalClearedRooms => "TotalClearedRooms",
                PrefixWord::TotalClearedWaves => "TotalClearedWaves",
                PrefixWord::Playtime => "Playtime",
            };

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(left_position),
                            top: Val::Px(top_position),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: true },
                    text: Text::with_section(
                        value,
                        TextStyle {
                            font: font.clone(),
                            font_size: 35.0,
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
                .insert(prefix.clone());
        }
    })
    .insert(Name::new("Texts"));
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
    .insert(Name::new("ReturnButton"))
    .insert(ResultSceneButton::Return);
}

fn save_profile_button(
    root: &mut ChildBuilder,
    scenes_materials: &ScenesMaterials,
    profile: Res<Profile>,
) {
    if profile.game_mode == GameMode::ClassicMode && !profile.is_run_completed {
        return;
    }

    let handle_image = scenes_materials.icon_materials.leaderboard.clone();

    let size = Size {
        width: Val::Px(BUTTON_SIDE),
        height: Val::Px(BUTTON_SIDE),
    };

    root.spawn_bundle(ButtonBundle {
        style: Style {
            position: Rect {
                left: Val::Px(550.0),
                top: Val::Px(440.0),
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
    .insert(Name::new("SaveProfileButton"))
    .insert(ResultSceneButton::SaveProfile);
}

fn play_again_button(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let handle_image = scenes_materials.icon_materials.restart.clone();

    let size = Size {
        width: Val::Px(BUTTON_SIDE),
        height: Val::Px(BUTTON_SIDE),
    };

    root.spawn_bundle(ButtonBundle {
        style: Style {
            position: Rect {
                left: Val::Px(400.0),
                top: Val::Px(440.0),
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
    .insert(Name::new("PlayAgainButton"))
    .insert(ResultSceneButton::PlayAgain);
}

fn button_handle_system(
    mut button_query: Query<
        (&ResultSceneButton, &Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut user_input_controller: ResMut<UserInputController>,
    mut state: ResMut<State<SceneState>>,
    mut string: Local<String>,
) {
    for (button, interaction, mut ui_image) in button_query.iter_mut() {
        match *button {
            ResultSceneButton::Return => match *interaction {
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
            ResultSceneButton::SaveProfile => match *interaction {
                Interaction::None => {
                    ui_image.0 = scenes_materials.icon_materials.leaderboard.clone()
                }
                Interaction::Hovered => {
                    ui_image.0 = scenes_materials.icon_materials.leaderboard_hovered.clone()
                }
                Interaction::Clicked => {
                    user_input_controller.0 = true;
                    string.clear();
                }
            },
            ResultSceneButton::PlayAgain => match *interaction {
                Interaction::None => ui_image.0 = scenes_materials.icon_materials.restart.clone(),
                Interaction::Hovered => {
                    ui_image.0 = scenes_materials.icon_materials.restart_hovered.clone()
                }
                Interaction::Clicked => {
                    state
                        .set(SceneState::GameModeSelectScene)
                        .expect("Couldn't switch state to Game Mode Select Scene");
                }
            },
        };
    }
}

fn user_input_text(grandparent: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());

    grandparent
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                size: Size::new(
                    Val::Px(USER_INPUT_NAME_BORDER_WIDTH),
                    Val::Px(USER_INPUT_NAME_BORDER_HEIGHT),
                ),
                position: Rect {
                    top: Val::Px((WINDOW_HEIGHT / 2.0) - (USER_INPUT_NAME_BORDER_HEIGHT / 2.0)),
                    left: Val::Px(
                        (WINDOW_HEIGHT * RESOLUTION) / 2.0 - (USER_INPUT_NAME_BORDER_WIDTH / 2.0),
                    ),
                    bottom: Val::Auto,
                    right: Val::Auto,
                },
                ..Default::default()
            },
            color: UiColor(Color::DARK_GRAY),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(UserInput)
                .insert(Name::new("UserInput"));
        })
        .insert(UserInputBox)
        .insert(Name::new("UserInputBox"));
}

fn user_input_visibility_handle(
    mut set: ParamSet<(
        Query<&mut Visibility, With<UserInputBox>>,
        Query<&mut Visibility, With<UserInput>>,
    )>,
    user_input_controller: Res<UserInputController>,
) {
    if user_input_controller.is_changed() {
        if user_input_controller.0 == true {
            for mut visibility in set.p0().iter_mut() {
                visibility.is_visible = true;
            }

            for mut visibility in set.p1().iter_mut() {
                visibility.is_visible = true;
            }
        } else {
            for mut visibility in set.p0().iter_mut() {
                visibility.is_visible = false;
            }

            for mut visibility in set.p1().iter_mut() {
                visibility.is_visible = false;
            }
        }
    }
}

fn user_input_handle(
    mut user_input_query: Query<&mut Text, With<UserInput>>,
    mut user_input_controller: ResMut<UserInputController>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut state: ResMut<State<SceneState>>,
    mut user_name: Local<String>,
    mut profile: ResMut<Profile>,
    keys: Res<Input<KeyCode>>,
) {
    if user_input_controller.0 {
        if keys.just_pressed(KeyCode::Return) {
            profile.set_name(user_name.clone());
            stored_profile(profile.convert_to_stored_profile());
            user_name.clear();
            state
                .set(SceneState::HighscoreScene)
                .expect("Couldn't switch state to HighscoreScene");
        }

        if keys.just_pressed(KeyCode::Escape) {
            user_input_controller.0 = false;
            user_name.clear();
        }

        if keys.just_pressed(KeyCode::Back) {
            user_name.pop();
        }

        if user_name.len() <= 12 {
            for ev in char_evr.iter() {
                let char = ev.char;
                if char.is_ascii() {
                    user_name.push(char);
                }
            }
        }

        let mut text = user_input_query.get_single_mut().unwrap();
        text.sections[0].value = user_name.to_string();
    }
}

fn stored_profile(profile: StoredProfile) {
    let mut profiles: Vec<StoredProfile> = match File::open(HIGHSCORE_FILE) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).expect("JSON was not well-formatted")
        }
        Err(err) => panic!("Can't find highscores file: {}", err.to_string()),
    };

    profiles.push(profile);

    let mut profiles_file = File::create(HIGHSCORE_FILE).expect("Can't open highscores file");
    let profiles_str: String = serde_json::to_string(&profiles).unwrap();
    profiles_file
        .write(profiles_str.as_bytes())
        .expect("Unable to write file");
}

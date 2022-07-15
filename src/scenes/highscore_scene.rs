use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use chrono::{DateTime, Datelike};
use std::fs::File;
use std::io::prelude::*;
use std::slice::Iter;

use crate::config::HIGHSCORE_FILE;
use crate::ingame::resources::fixed::gender::Gender;
use crate::ingame::resources::fixed::hero_class::HeroClass;
use crate::ingame::resources::{game_mode::GameMode, stored_profile::StoredProfile};
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::materials::scenes::ScenesMaterials;
use crate::resources::materials::Materials;
use crate::resources::tile_size::TileSize;
use crate::scenes::SceneState;

const BOOK_TILE_SIZE: TileSize = TileSize {
    width: 190.0,
    height: 160.0,
};

const HERO_IMAGE_SIZE: TileSize = TileSize {
    width: 16.0 * 6.0,
    height: 28.0 * 6.0,
};

const HIGHSCORE_SCENE_BUTTON_SIDE: f32 = 50.0;

const HIGHSCORE_SCENE_BUTTON_POSITIONS: [Rect<Val>; 3] = [
    Rect {
        left: Val::Px(HIGHSCORE_SCENE_BUTTON_SIDE / 2.0),
        top: Val::Px(HIGHSCORE_SCENE_BUTTON_SIDE / 2.0),
        right: Val::Auto,
        bottom: Val::Auto,
    },
    Rect {
        left: Val::Auto,
        top: Val::Px(100.0),
        right: Val::Px(285.0),
        bottom: Val::Auto,
    },
    Rect {
        left: Val::Px(200.0),
        top: Val::Px(100.0),
        bottom: Val::Auto,
        right: Val::Auto,
    },
];

#[derive(Component, Copy, Clone)]
enum HighscoreSceneButton {
    Return,
    Next,
    Previous,
}

impl HighscoreSceneButton {
    pub fn iterator() -> Iter<'static, HighscoreSceneButton> {
        static HIGHSCORE_SCENE_BUTTONS: [HighscoreSceneButton; 3] = [
            HighscoreSceneButton::Return,
            HighscoreSceneButton::Next,
            HighscoreSceneButton::Previous,
        ];
        HIGHSCORE_SCENE_BUTTONS.iter()
    }
}

#[derive(Component, Copy, Clone)]
enum PrefixWord {
    Name,
    Gender,
    GameMode,
    TotalKilledMonsters,
    TotalClearedRooms,
    TotalClearedWaves,
    Date,
    Playtime,
}

impl PrefixWord {
    pub fn iterator() -> Iter<'static, PrefixWord> {
        static PREFIX_WORDS: [PrefixWord; 8] = [
            PrefixWord::Name,
            PrefixWord::Gender,
            PrefixWord::GameMode,
            PrefixWord::TotalKilledMonsters,
            PrefixWord::TotalClearedRooms,
            PrefixWord::TotalClearedWaves,
            PrefixWord::Date,
            PrefixWord::Playtime,
        ];
        PREFIX_WORDS.iter()
    }
}

#[derive(Component)]
pub struct HighscoreBook {
    current_page: isize,
    total_pages: usize,
    is_reverse: bool,
    timer: AnimationTimer,
    animation_indexes: Vec<usize>,
    animation_index: usize,
    profiles: Vec<StoredProfile>,
}

#[derive(Component)]
struct HeroImage;

#[derive(Component)]
struct TextsNode;

#[derive(Component)]
struct AnimationTimer(Timer);

struct HighscoreSceneData {
    user_interface_root: Entity,
    background: Entity,
    book: Entity,
}

pub struct HighscoreScenePlugin;

impl Plugin for HighscoreScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::HighscoreScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::HighscoreScene)
                .with_system(button_handle_system)
                .with_system(book_animation_handle_system)
                .with_system(hero_image_handle_system)
                .with_system(texts_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::HighscoreScene).with_system(cleanup));
    }
}

fn cleanup(mut commands: Commands, highscore_scene_data: Res<HighscoreSceneData>) {
    commands
        .entity(highscore_scene_data.background)
        .despawn_recursive();

    commands
        .entity(highscore_scene_data.book)
        .despawn_recursive();

    commands
        .entity(highscore_scene_data.user_interface_root)
        .despawn_recursive();
}

fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    scenes_materials: Res<ScenesMaterials>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dictionary: Res<Dictionary>,
) {
    // background
    let background = commands
        .spawn_bundle(SpriteBundle {
            texture: materials.sub_menu_background.clone(),
            ..Default::default()
        })
        .id();

    // book texture
    let book_tileset = scenes_materials.book_tileset.clone();
    let texture_atlas = TextureAtlas::from_grid(
        book_tileset,
        Vec2::new(BOOK_TILE_SIZE.width, BOOK_TILE_SIZE.width),
        7,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // profiles
    let profiles: Vec<StoredProfile> = match File::open(HIGHSCORE_FILE) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).expect("JSON was not well-formatted")
        }
        Err(err) => panic!("Can't find highscores file: {}", err.to_string()),
    };

    // book
    let book = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(-25.0, -30.0, 1.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(BOOK_TILE_SIZE.width, BOOK_TILE_SIZE.height)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HighscoreBook {
            current_page: -1,
            total_pages: profiles.len(),
            timer: AnimationTimer(Timer::from_seconds(0.1, true)),
            animation_indexes: Vec::new(),
            animation_index: 0,
            is_reverse: false,
            profiles,
        })
        .id();

    // user interface root
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            buttons(parent, &scenes_materials);
            hero_image(parent);
            texts(parent, &materials, dictionary.clone())
        })
        .id();

    commands.insert_resource(HighscoreSceneData {
        user_interface_root,
        background,
        book,
    });
}

fn buttons(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    for (index, button) in HighscoreSceneButton::iterator().enumerate() {
        let handle_image = match button {
            HighscoreSceneButton::Return => {
                scenes_materials.icon_materials.home_icon_normal.clone()
            }
            _ => scenes_materials.icon_materials.home_icon_normal.clone(),
        };

        let size = match button {
            HighscoreSceneButton::Return => Size {
                width: Val::Px(HIGHSCORE_SCENE_BUTTON_SIDE),
                height: Val::Px(HIGHSCORE_SCENE_BUTTON_SIDE),
            },
            _ => Size {
                width: Val::Px(250.0),
                height: Val::Px(320.0),
            },
        };

        match button {
            HighscoreSceneButton::Return => {
                root.spawn_bundle(ButtonBundle {
                    style: Style {
                        position: HIGHSCORE_SCENE_BUTTON_POSITIONS[index],
                        size,
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
                        position: HIGHSCORE_SCENE_BUTTON_POSITIONS[index],
                        size,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .insert(button.clone());
            }
        };
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &HighscoreSceneButton, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    mut highscore_book_query: Query<&mut HighscoreBook>,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            HighscoreSceneButton::Return => match *interaction {
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
            HighscoreSceneButton::Next => {
                if *interaction == Interaction::Clicked {
                    let mut highscore_book = highscore_book_query.get_single_mut().unwrap();
                    let total_pages = highscore_book.total_pages as isize;
                    if highscore_book.animation_indexes.len() == 0 {
                        highscore_book.is_reverse = false;
                        highscore_book.animation_index = 0;
                        if highscore_book.current_page == -1 {
                            highscore_book.animation_indexes = [0, 1, 2, 3].to_vec();
                        } else if highscore_book.current_page < total_pages - 1 {
                            highscore_book.animation_indexes = [3, 4, 5, 6, 3].to_vec();
                        } else if highscore_book.current_page == total_pages - 1 {
                            highscore_book.animation_indexes = [3, 2, 1, 0].to_vec();
                        }
                    }
                }
            }
            HighscoreSceneButton::Previous => {
                if *interaction == Interaction::Clicked {
                    let mut highscore_book = highscore_book_query.get_single_mut().unwrap();
                    if highscore_book.animation_indexes.len() == 0 {
                        highscore_book.is_reverse = true;
                        highscore_book.animation_index = 0;
                        if highscore_book.current_page == 0 {
                            highscore_book.animation_indexes = [3, 2, 1, 0].to_vec();
                        } else if highscore_book.current_page > 0 {
                            highscore_book.animation_indexes = [3, 6, 5, 4, 3].to_vec();
                        }
                    }
                }
            }
        }
    }
}

fn book_animation_handle_system(
    time: Res<Time>,
    mut query: Query<(&mut HighscoreBook, &mut TextureAtlasSprite)>,
) {
    for (mut highscore_book, mut sprite) in query.iter_mut() {
        if highscore_book.animation_indexes.len() != 0 {
            highscore_book.timer.0.tick(time.delta());
            if highscore_book.timer.0.just_finished() {
                sprite.index = highscore_book.animation_indexes[highscore_book.animation_index];
                highscore_book.animation_index += 1;
                if highscore_book.animation_index == highscore_book.animation_indexes.len() {
                    highscore_book.animation_indexes = Vec::new();
                    highscore_book.animation_index = 0;

                    if highscore_book.is_reverse {
                        highscore_book.current_page -= 1;
                    } else {
                        highscore_book.current_page += 1;
                    }

                    let total_pages = highscore_book.total_pages as isize;
                    if highscore_book.current_page > total_pages - 1 {
                        highscore_book.current_page = -1;
                    }
                }
            }
        }
    }
}

fn hero_image(root: &mut ChildBuilder) {
    root.spawn_bundle(ImageBundle {
        style: Style {
            position: Rect {
                right: Val::Auto,
                bottom: Val::Auto,
                left: Val::Px(280.0),
                top: Val::Px(100.0),
            },
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(HERO_IMAGE_SIZE.width),
                Val::Px(HERO_IMAGE_SIZE.height),
            ),
            ..Default::default()
        },
        visibility: Visibility { is_visible: false },
        ..Default::default()
    })
    .insert(HeroImage);
}

fn hero_image_handle_system(
    mut query: Query<(&HeroImage, &mut UiImage, &mut Visibility)>,
    mut highscore_book_query: Query<&mut HighscoreBook>,
    scenes_materials: Res<ScenesMaterials>,
) {
    for (_hero_image, mut ui_image, mut visibility) in query.iter_mut() {
        let highscore_book = highscore_book_query.get_single_mut().unwrap();
        if highscore_book.current_page != -1 && highscore_book.animation_indexes.len() == 0 {
            let index = highscore_book.current_page.clone() as usize;
            ui_image.0 = match highscore_book.profiles[index].hero_class {
                HeroClass::Elf => match highscore_book.profiles[index].gender {
                    Gender::Male => scenes_materials.heros_materials.male_elf.clone(),
                    Gender::Female => scenes_materials.heros_materials.female_elf.clone(),
                },
                HeroClass::Knight => match highscore_book.profiles[index].gender {
                    Gender::Male => scenes_materials.heros_materials.male_knight.clone(),
                    Gender::Female => scenes_materials.heros_materials.female_knight.clone(),
                },
                HeroClass::Lizard => match highscore_book.profiles[index].gender {
                    Gender::Male => scenes_materials.heros_materials.male_lizard.clone(),
                    Gender::Female => scenes_materials.heros_materials.female_lizard.clone(),
                },
                HeroClass::Wizard => match highscore_book.profiles[index].gender {
                    Gender::Male => scenes_materials.heros_materials.male_wizard.clone(),
                    Gender::Female => scenes_materials.heros_materials.female_wizard.clone(),
                },
            };
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}

fn texts(root: &mut ChildBuilder, materials: &Materials, dictionary: Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let position_of_texts: [[f32; 2]; 8] = [
        [210.0, 300.0],
        [210.0, 340.0],
        [210.0, 380.0],
        [500.0, 140.0],
        [500.0, 180.0],
        [500.0, 220.0],
        [500.0, 260.0],
        [500.0, 300.0],
    ];

    root.spawn_bundle(NodeBundle {
        focus_policy: FocusPolicy::Pass,
        style: Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        for (index, prevalue) in PrefixWord::iterator().enumerate() {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(position_of_texts[index][0]),
                            top: Val::Px(position_of_texts[index][1]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: true },
                    text: Text::with_section(
                        "",
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
                })
                .insert(prevalue.clone());
        }
    })
    .insert(TextsNode);
}

fn texts_handle_system(
    mut query: Query<(&TextsNode, &mut Style, &mut Children)>,
    mut text_type_query: Query<&PrefixWord>,
    mut text_query: Query<&mut Text>,
    mut highscore_book_query: Query<&mut HighscoreBook>,
    dictionary: Res<Dictionary>,
) {
    for (_hero_image, mut style, children) in query.iter_mut() {
        let highscore_book = highscore_book_query.get_single_mut().unwrap();
        if highscore_book.current_page != -1 && highscore_book.animation_indexes.len() == 0 {
            let profile_index = highscore_book.current_page.clone() as usize;

            let glossary = dictionary.get_glossary();

            for text_index in 0..children.len() {
                let text_value = match text_type_query.get_mut(children[text_index]).unwrap() {
                    PrefixWord::Name => {
                        let prefix = glossary.highscore_scene_text.name.clone();
                        let value = highscore_book.profiles[profile_index].name.clone();
                        prefix + value.as_str()
                    }
                    PrefixWord::Gender => {
                        let prefix = glossary.highscore_scene_text.gender.clone();
                        let gender = highscore_book.profiles[profile_index].gender.clone();
                        let value = match gender {
                            Gender::Female => glossary.shared_text.female.clone(),
                            Gender::Male => glossary.shared_text.male.clone(),
                        };
                        prefix + value.as_str()
                    }
                    PrefixWord::GameMode => {
                        let game_mode = highscore_book.profiles[profile_index].game_mode.clone();
                        let value = match game_mode {
                            GameMode::ClassicMode => glossary.shared_text.classic_mode.clone(),
                            GameMode::SurvivalMode => glossary.shared_text.survival_mode.clone(),
                        };
                        value
                    }
                    PrefixWord::TotalKilledMonsters => {
                        let prefix = glossary.highscore_scene_text.total_killed_monsters.clone();
                        let value = highscore_book.profiles[profile_index].total_killed_monsters;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWord::TotalClearedRooms => {
                        let prefix = glossary.highscore_scene_text.total_cleared_rooms.clone();
                        let value = highscore_book.profiles[profile_index].total_cleared_rooms;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWord::TotalClearedWaves => {
                        let prefix = glossary.highscore_scene_text.total_cleared_waves.clone();
                        let value = highscore_book.profiles[profile_index].total_cleared_waves;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWord::Date => {
                        let prefix = glossary.highscore_scene_text.date.clone();
                        let date_str = highscore_book.profiles[profile_index].date.clone();
                        let date = DateTime::parse_from_rfc3339(date_str.as_str())
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
                    PrefixWord::Playtime => {
                        let prefix = glossary.highscore_scene_text.playtime.clone();
                        let playtime = highscore_book.profiles[profile_index].playtime;

                        let seconds = playtime % 60;
                        let formated_seconds = match seconds {
                            0..=9 => format!("0{}", seconds),
                            _ => format!("{}", seconds),
                        };

                        let minutes = (playtime / 60) % 60;
                        let formated_minutes = match minutes {
                            0..=9 => format!("0{}", minutes),
                            _ => format!("{}", minutes),
                        };

                        let hours = (playtime / 60) / 60;
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

                let mut text = text_query.get_mut(children[text_index]).unwrap();
                text.sections[0].value = text_value;
            }
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}

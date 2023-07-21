use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use chrono::{DateTime, Datelike};
use std::fs::File;
use std::io::prelude::*;
use std::slice::Iter;

use crate::config::HIGHSCORE_FILE;
use crate::materials::font::FontMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::language::Language;
use crate::resources::tile_size::TileSize;
use crate::resources::{game_mode::GameMode, stored_profile::StoredProfile};
use crate::scenes::SceneState;

const BOOK_TILE_SIZE: TileSize = TileSize {
    width: 190.0,
    height: 160.0,
};

const HERO_IMAGE_SIZE: TileSize = TileSize {
    width: 16.0 * 6.0,
    height: 28.0 * 6.0,
};

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Return,
    Next,
    Previous,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Return,
            ButtonComponent::Next,
            ButtonComponent::Previous,
        ]
        .iter()
    }
}

#[derive(Component, Copy, Clone)]
enum PrefixWordComponent {
    Name,
    Gender,
    GameMode,
    TotalKilledMonsters,
    TotalClearedRooms,
    TotalClearedWaves,
    Date,
    Playtime,
}

impl PrefixWordComponent {
    pub fn iterator() -> Iter<'static, PrefixWordComponent> {
        [
            PrefixWordComponent::Name,
            PrefixWordComponent::Gender,
            PrefixWordComponent::GameMode,
            PrefixWordComponent::TotalKilledMonsters,
            PrefixWordComponent::TotalClearedRooms,
            PrefixWordComponent::TotalClearedWaves,
            PrefixWordComponent::Date,
            PrefixWordComponent::Playtime,
        ]
        .iter()
    }
}

#[derive(Component)]
pub struct HighscoreBookComponent {
    current_page: isize,
    total_pages: usize,
    is_reverse: bool,
    timer: Timer,
    animation_indexes: Vec<usize>,
    animation_index: usize,
    profiles: Vec<StoredProfile>,
}

#[derive(Component)]
struct HeroImageComponent;

#[derive(Component)]
struct TextsNodeComponent;

#[derive(Resource)]
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
    font_materials: Res<FontMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dictionary: Res<Dictionary>,
) {
    // background
    let background = commands
        .spawn(SpriteBundle {
            texture: scenes_materials.sub_background_image.clone(),
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
        None,
        None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // profiles
    let profiles: Vec<StoredProfile> = match File::open(HIGHSCORE_FILE) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).expect("JSON was not well-formatted")
        }
        Err(err) => panic!("Can't find highscores file: {}", err),
    };

    // book
    let book = commands
        .spawn(SpriteSheetBundle {
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
        .insert(HighscoreBookComponent {
            current_page: -1,
            total_pages: profiles.len(),
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            animation_indexes: Vec::new(),
            animation_index: 0,
            is_reverse: false,
            profiles,
        })
        .id();

    // user interface root
    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            buttons(parent, &scenes_materials);
            hero_image(parent);
            texts(parent, &font_materials, dictionary.clone())
        })
        .id();

    commands.insert_resource(HighscoreSceneData {
        user_interface_root,
        background,
        book,
    });
}

fn buttons(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let positions: [UiRect; 3] = [
        UiRect {
            left: Val::Px(50.0 / 2.0),
            top: Val::Px(50.0 / 2.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        UiRect {
            left: Val::Auto,
            top: Val::Px(100.0),
            right: Val::Px(285.0),
            bottom: Val::Auto,
        },
        UiRect {
            left: Val::Px(200.0),
            top: Val::Px(100.0),
            bottom: Val::Auto,
            right: Val::Auto,
        },
    ];

    for (index, button) in ButtonComponent::iterator().enumerate() {
        match button {
            ButtonComponent::Return => {
                let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();

                let size = Size {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                };
                root.spawn(ButtonBundle {
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
                .insert(*button);
            }
            _ => {
                let size = Size {
                    width: Val::Px(250.0),
                    height: Val::Px(320.0),
                };

                root.spawn(ButtonBundle {
                    style: Style {
                        position: positions[index],
                        size,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                })
                .insert(*button);
            }
        };
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    mut highscore_book_query: Query<&mut HighscoreBookComponent>,
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
            ButtonComponent::Next => {
                if *interaction == Interaction::Clicked {
                    let mut highscore_book = highscore_book_query.get_single_mut().unwrap();
                    let total_pages = highscore_book.total_pages as isize;
                    if highscore_book.animation_indexes.is_empty() {
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
            ButtonComponent::Previous => {
                if *interaction == Interaction::Clicked {
                    let mut highscore_book = highscore_book_query.get_single_mut().unwrap();
                    if highscore_book.animation_indexes.is_empty() {
                        highscore_book.is_reverse = true;
                        highscore_book.animation_index = 0;
                        highscore_book.animation_indexes = match highscore_book.current_page {
                            0 => [3, 2, 1, 0].to_vec(),
                            _ => [3, 6, 5, 4, 3].to_vec(),
                        }
                    }
                }
            }
        }
    }
}

fn book_animation_handle_system(
    mut query: Query<(&mut HighscoreBookComponent, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut highscore_book, mut sprite) in query.iter_mut() {
        if !highscore_book.animation_indexes.is_empty() {
            highscore_book.timer.tick(time.delta());
            if highscore_book.timer.just_finished() {
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
    root.spawn(ImageBundle {
        style: Style {
            position: UiRect {
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
    .insert(HeroImageComponent);
}

fn hero_image_handle_system(
    mut query: Query<(&HeroImageComponent, &mut UiImage, &mut Visibility)>,
    mut highscore_book_query: Query<&mut HighscoreBookComponent>,
    scenes_materials: Res<ScenesMaterials>,
) {
    for (_hero_image, mut ui_image, mut visibility) in query.iter_mut() {
        let highscore_book = highscore_book_query.get_single_mut().unwrap();
        if highscore_book.current_page != -1 && highscore_book.animation_indexes.is_empty() {
            let index = highscore_book.current_page as usize;
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

fn texts(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
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

    root.spawn(NodeBundle {
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
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        for (index, prevalue) in PrefixWordComponent::iterator().enumerate() {
            parent
                .spawn(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(position_of_texts[index][0]),
                            top: Val::Px(position_of_texts[index][1]),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    visibility: Visibility { is_visible: true },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 25.0,
                            color: Color::BLACK,
                        }
                    ).with_alignment(
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        }
                    ),
                    ..Default::default()
                })
                .insert(*prevalue);
        }
    })
    .insert(TextsNodeComponent);
}

fn texts_handle_system(
    mut query: Query<(&TextsNodeComponent, &mut Style, &mut Children)>,
    mut highscore_book_query: Query<&mut HighscoreBookComponent>,
    mut text_type_query: Query<&PrefixWordComponent>,
    mut text_query: Query<&mut Text>,
    dictionary: Res<Dictionary>,
) {
    for (_hero_image, mut style, children) in query.iter_mut() {
        let highscore_book = highscore_book_query.get_single_mut().unwrap();
        if highscore_book.current_page != -1 && highscore_book.animation_indexes.is_empty() {
            let profile_index = highscore_book.current_page as usize;

            let glossary = dictionary.get_glossary();

            for text_index in 0..children.len() {
                let text_value = match text_type_query.get_mut(children[text_index]).unwrap() {
                    PrefixWordComponent::Name => {
                        let prefix = glossary.highscore_scene_text.name.clone();
                        let value = highscore_book.profiles[profile_index].name.clone();
                        prefix + value.as_str()
                    }
                    PrefixWordComponent::Gender => {
                        let prefix = glossary.highscore_scene_text.gender.clone();
                        let gender = highscore_book.profiles[profile_index].gender.clone();
                        let value = match gender {
                            Gender::Female => glossary.shared_text.female.clone(),
                            Gender::Male => glossary.shared_text.male.clone(),
                        };
                        prefix + value.as_str()
                    }
                    PrefixWordComponent::GameMode => {
                        let game_mode = highscore_book.profiles[profile_index].game_mode.clone();
                        match game_mode {
                            GameMode::ClassicMode => glossary.shared_text.classic_mode.clone(),
                            GameMode::SurvivalMode => glossary.shared_text.survival_mode.clone(),
                        }
                    }
                    PrefixWordComponent::TotalKilledMonsters => {
                        let prefix = glossary.highscore_scene_text.total_killed_monsters.clone();
                        let value = highscore_book.profiles[profile_index].total_killed_monsters;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWordComponent::TotalClearedRooms => {
                        let prefix = glossary.highscore_scene_text.total_cleared_rooms.clone();
                        let value = highscore_book.profiles[profile_index].total_cleared_rooms;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWordComponent::TotalClearedWaves => {
                        let prefix = glossary.highscore_scene_text.total_cleared_waves.clone();
                        let value = highscore_book.profiles[profile_index].total_cleared_waves;
                        prefix + value.to_string().as_str()
                    }
                    PrefixWordComponent::Date => {
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
                    PrefixWordComponent::Playtime => {
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

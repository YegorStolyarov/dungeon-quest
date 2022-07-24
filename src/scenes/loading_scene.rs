use bevy::prelude::*;

use crate::config::*;
use crate::scenes::SceneState;

use crate::ingame::materials::{
    dungeon::DungeonMaterials, heros::HerosMaterials, weapons::WeaponsMaterials, InGameMaterials,
};
use crate::ingame::resources::dungeon::rooms::Rooms;
use crate::ingame::resources::data::Data;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::materials::{
    scenes::{FlagMaterials, IconMaterials, MenuBoxMaterials, ScenesMaterials},
    Materials,
};

const LOADING_TEXT_FONT_SIZE: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 40.0;

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct Loader {
    max_width: f32,
    current_width: f32,
}

struct LoadingSceneData {
    user_interface_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::LoadingScene)
                .with_system(setup)
                .with_system(load_materials)
                .with_system(load_data),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::LoadingScene).with_system(update_loader),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::LoadingScene).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, dictionary: Res<Dictionary>) {
    let user_interface_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            loading_text(parent, &asset_server, &dictionary);
            loader_bundle(parent, &asset_server, &dictionary);
        })
        .id();

    commands.insert_resource(LoadingSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, loading_scene_data: Res<LoadingSceneData>) {
    commands
        .entity(loading_scene_data.user_interface_root)
        .despawn_recursive();
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: UiColor(Color::BLACK),
        ..Default::default()
    }
}

fn loader_bundle(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn_bundle(
        // Border
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::new(
                    Val::Px(LOADING_BORDER_WIDTH),
                    Val::Px(LOADING_BORDER_HEIGHT),
                ),
                position: Rect {
                    top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                    left: Val::Px(
                        (WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0),
                    ),
                    bottom: Val::Auto,
                    right: Val::Auto,
                },
                ..Default::default()
            },
            color: UiColor(Color::DARK_GRAY),
            ..Default::default()
        },
    )
    .with_children(|parent| {
        // Loader
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    size: Size::new(
                        Val::Px(0.0),
                        Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    ),
                    position: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                color: UiColor(Color::rgb(247.0 / 255.0, 104.0 / 255.0, 12.0 / 255.0)),
                ..Default::default()
            })
            .with_children(|parent| {
                let font_str = match dictionary.get_current_language() {
                    Language::VI => ROBOTO_FONT,
                    Language::EN => FIBBERISH_FONT,
                };

                parent.spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load(font_str),
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(Loader {
                max_width: LOADING_BORDER_WIDTH - 10.0,
                current_width: 0.0,
            });
    });
}

fn loading_text(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(LOADING_BORDER_WIDTH), Val::Px(35.0)),
            position: Rect {
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
                top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossary = dictionary.get_glossary();

        let font_str = match dictionary.get_current_language() {
            Language::VI => ROBOTO_FONT,
            Language::EN => FIBBERISH_FONT,
        };

        parent.spawn_bundle(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },

            text: Text::with_section(
                glossary.loading_scene_text.loading.to_string(),
                TextStyle {
                    font: asset_server.load(font_str),
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    });
}

fn update_loader(
    mut query: Query<(&mut Loader, &mut Style, &Children)>,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<SceneState>>,
) {
    for (mut loader, mut style, children) in query.iter_mut() {
        if loader.current_width < loader.max_width {
            loader.current_width += 100.0;
            style.size.width = Val::Px(loader.current_width);

            let value = (loader.current_width / loader.max_width * 100.0) as usize;
            if value >= 6 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = value.to_string() + "%";
            }
        } else {
            state
                .set(SceneState::MainMenuScene)
                .expect("Couldn't switch state to Main Menu Scene");
        }
    }
}

fn load_materials(mut commands: Commands, asset_server: Res<AssetServer>) {
    let materials: Materials = Materials {
        roboto_font: asset_server.load(ROBOTO_FONT),
        fibberish_font: asset_server.load(FIBBERISH_FONT),
        main_menu_background: asset_server.load(MAIN_MENU_BACKGROUND_IMAGE),
        sub_menu_background: asset_server.load(SUB_MENU_BACKGROUND_IMAGE),
    };

    let scenes_materials: ScenesMaterials = ScenesMaterials {
        menu_box_materials: MenuBoxMaterials {
            top_right: asset_server.load("scenes/gui/menu_box/top_right.png"),
            top_center: asset_server.load("scenes/gui/menu_box/top_center.png"),
            top_left: asset_server.load("scenes/gui/menu_box/top_left.png"),
            mid_right: asset_server.load("scenes/gui/menu_box/mid_right.png"),
            mid_center: asset_server.load("scenes/gui/menu_box/mid_center.png"),
            mid_left: asset_server.load("scenes/gui/menu_box/mid_left.png"),
            bottom_right: asset_server.load("scenes/gui/menu_box/bottom_right.png"),
            bottom_center: asset_server.load("scenes/gui/menu_box/bottom_center.png"),
            bottom_left: asset_server.load("scenes/gui/menu_box/bottom_left.png"),
        },
        icon_materials: IconMaterials {
            home_icon_normal: asset_server.load("icons/home_icon_normal.png"),
            home_icon_hovered: asset_server.load("icons/home_icon_hovered.png"),
            home_icon_clicked: asset_server.load("icons/home_icon_clicked.png"),
            music_icon_on: asset_server.load("icons/music_icon_on.png"),
            music_icon_off: asset_server.load("icons/music_icon_off.png"),
            music_icon_hovered: asset_server.load("icons/music_icon_hovered.png"),
            sound_icon_on: asset_server.load("icons/sound_icon_on.png"),
            sound_icon_off: asset_server.load("icons/sound_icon_off.png"),
            sound_icon_hovered: asset_server.load("icons/sound_icon_hovered.png"),
            leaderboard: asset_server.load("icons/leaderboard.png"),
            leaderboard_hovered: asset_server.load("icons/leaderboard_hovered.png"),
            restart: asset_server.load("icons/restart.png"),
            restart_hovered: asset_server.load("icons/restart_hovered.png"),
        },
        book_tileset: asset_server.load("scenes/book.png"),
        heros_materials: HerosMaterials {
            male_elf: asset_server.load("scenes/heros/male_elf.png"),
            male_knight: asset_server.load("scenes/heros/male_knight.png"),
            male_wizard: asset_server.load("scenes/heros/male_wizard.png"),
            male_lizard: asset_server.load("scenes/heros/male_lizard.png"),
            female_elf: asset_server.load("scenes/heros/female_elf.png"),
            female_knight: asset_server.load("scenes/heros/female_knight.png"),
            female_wizard: asset_server.load("scenes/heros/female_wizard.png"),
            female_lizard: asset_server.load("scenes/heros/female_lizard.png"),
        },
        flag_materials: FlagMaterials {
            vietnam: asset_server.load("scenes/vietnam.png"),
            united_states: asset_server.load("scenes/united_states.png"),
        },
    };

    let ingame_materials: InGameMaterials = InGameMaterials {
        heros_materials: HerosMaterials {
            male_elf: asset_server.load("ingame/heros/male_elf.png"),
            male_knight: asset_server.load("ingame/heros/male_knight.png"),
            male_wizard: asset_server.load("ingame/heros/male_wizard.png"),
            male_lizard: asset_server.load("ingame/heros/male_lizard.png"),
            female_elf: asset_server.load("ingame/heros/female_elf.png"),
            female_knight: asset_server.load("ingame/heros/female_knight.png"),
            female_wizard: asset_server.load("ingame/heros/female_wizard.png"),
            female_lizard: asset_server.load("ingame/heros/female_lizard.png"),
        },
        weapons_materials: WeaponsMaterials {
            bow: asset_server.load("ingame/weapons/bow.png"),
            arrow: asset_server.load("ingame/weapons/arrow.png"),
            short_sword: asset_server.load("ingame/weapons/short_sword.png"),
            sword: asset_server.load("ingame/weapons/sword.png"),
            machete: asset_server.load("ingame/weapons/machete.png"),
            small_hammer: asset_server.load("ingame/weapons/small_hammer.png"),
            mace: asset_server.load("ingame/weapons/mace.png"),
            big_hammer: asset_server.load("ingame/weapons/big_hammer.png"),
            small_wand: asset_server.load("ingame/weapons/small_wand.png"),
            magic_wand: asset_server.load("ingame/weapons/magic_wand.png"),
            magic_sword: asset_server.load("ingame/weapons/magic_sword.png"),
            spear: asset_server.load("ingame/weapons/spear.png"),
        },
        dungeon_materials: DungeonMaterials {
            floor: asset_server.load("ingame/dungeon/floor.png"),
            ladder: asset_server.load("ingame/dungeon/ladder.png"),
            wall: asset_server.load("ingame/dungeon/wall.png"),
            treasure: asset_server.load("ingame/dungeon/treasure.png"),
            wall_border_mid: asset_server.load("ingame/dungeon/wall_border_mid.png"),
            wall_border_corner_top_left: asset_server
                .load("ingame/dungeon/wall_border_corner_top_left.png"),
            wall_border_corner_top_right: asset_server
                .load("ingame/dungeon/wall_border_corner_top_right.png"),
            wall_border_corner_bottom_left: asset_server
                .load("ingame/dungeon/wall_border_corner_bottom_left.png"),
            wall_border_corner_bottom_right: asset_server
                .load("ingame/dungeon/wall_border_corner_bottom_right.png"),
            wall_border_left: asset_server.load("ingame/dungeon/wall_border_left.png"),
            wall_border_right: asset_server.load("ingame/dungeon/wall_border_right.png"),
            wall_border_corner_left: asset_server
                .load("ingame/dungeon/wall_border_corner_left.png"),
            wall_border_corner_right: asset_server
                .load("ingame/dungeon/wall_border_corner_right.png"),
            wall_left: asset_server.load("ingame/dungeon/wall_left.png"),
            wall_right: asset_server.load("ingame/dungeon/wall_right.png"),
            door_opened: asset_server.load("ingame/dungeon/door_opened.png"),
            door_closed: asset_server.load("ingame/dungeon/door_closed.png"),
            door_top_part: asset_server.load("ingame/dungeon/door_top_part.png"),
            door_left_part: asset_server.load("ingame/dungeon/door_left_part.png"),
            door_right_part: asset_server.load("ingame/dungeon/door_right_part.png"),
        },
    };

    commands.insert_resource(materials);
    commands.insert_resource(scenes_materials);
    commands.insert_resource(ingame_materials);
}

fn load_data(mut commands: Commands) {
    commands.insert_resource(Data::new());
    commands.insert_resource(Rooms::new());
}

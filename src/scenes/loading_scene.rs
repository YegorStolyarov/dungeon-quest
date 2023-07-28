use bevy::prelude::*;

use crate::config::*;
use crate::scenes::SceneState;

use crate::materials::bullets::BulletsMaterials;
use crate::materials::dungeon::DungeonMaterials;
use crate::materials::flag::FlagMaterials;
use crate::materials::font::FontMaterials;
use crate::materials::hearts::HeartsMaterials;
use crate::materials::heroes::HeroesMaterials;
use crate::materials::icon::IconMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::monsters::MonstersMaterials;
use crate::materials::potions::PotionsMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::materials::weapons::WeaponsMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::dungeon::rooms::Rooms;
use crate::resources::game_data::{GameData, PauseFlag};
use crate::resources::language::Language;

const LOADING_TEXT_FONT_SIZE: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 40.0;

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct LoaderComponent {
    max_width: f32,
    current_width: f32,
}

#[derive(Resource)]
struct LoadingSceneData {
    user_interface_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::LoadingScene), setup);
        app.add_systems(Update, (
            load_materials,
            load_data,
            update_loader
        ).run_if(in_state(SceneState::LoadingScene)));
        app.add_systems(OnExit(SceneState::LoadingScene), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, dictionary: Res<Dictionary>) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
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

fn loader_bundle(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn(
        // Border
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                width: Val::Px(LOADING_BORDER_WIDTH),
                height: Val::Px(LOADING_BORDER_HEIGHT),
                top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                left: Val::Px(
                    (WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0),
                ),
                bottom: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        },
    )
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    width: Val::Px(0.0),
                    height: Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    left: Val::Px(5.0),
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(247.0 / 255.0, 104.0 / 255.0, 12.0 / 255.0)),
                ..Default::default()
            })
            .with_children(|parent| {
                let font_str = match dictionary.get_current_language() {
                    Language::VI => ROBOTO_FONT,
                    Language::EN => FIBBERISH_FONT,
                };

                parent.spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load(font_str),
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        }
                    ).with_alignment(
                        TextAlignment::Center
                    ),
                    ..Default::default()
                });
            })
            .insert(LoaderComponent {
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
    root.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            width: Val::Px(LOADING_BORDER_WIDTH),
            height: Val::Px(35.0),
            left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
            top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
            bottom: Val::Auto,
            right: Val::Auto,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossary = dictionary.get_glossary();

        let font_str = match dictionary.get_current_language() {
            Language::VI => ROBOTO_FONT,
            Language::EN => FIBBERISH_FONT,
        };

        parent.spawn(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },

            text: Text::from_section(
                glossary.loading_scene_text.loading,
                TextStyle {
                    font: asset_server.load(font_str),
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                }
            ).with_alignment(
                TextAlignment::Center
            ),
            ..Default::default()
        });
    });
}

fn update_loader(
    mut query: Query<(&mut LoaderComponent, &mut Style, &Children)>,
    mut state: ResMut<NextState<SceneState>>,
    mut text_query: Query<&mut Text>,
) {
    for (mut loader, mut style, children) in query.iter_mut() {
        if loader.current_width < loader.max_width {
            loader.current_width += 2.5;
            style.width = Val::Px(loader.current_width);

            let value = (loader.current_width / loader.max_width * 100.0) as usize;
            if value >= 6 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = value.to_string() + "%";
            }
        } else {
            state
                .set(SceneState::MainMenuScene);
        }
    }
}

fn load_materials(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_materials: FontMaterials = FontMaterials {
        roboto_font: asset_server.load(ROBOTO_FONT),
        fibberish_font: asset_server.load(FIBBERISH_FONT),
    };

    let scenes_materials: ScenesMaterials = ScenesMaterials {
        main_background_image: asset_server.load(MAIN_MENU_BACKGROUND_IMAGE),
        sub_background_image: asset_server.load(SUB_MENU_BACKGROUND_IMAGE),
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
        heroes_materials: HeroesMaterials {
            male_elf: asset_server.load("scenes/heroes/male_elf.png"),
            male_knight: asset_server.load("scenes/heroes/male_knight.png"),
            male_wizard: asset_server.load("scenes/heroes/male_wizard.png"),
            male_lizard: asset_server.load("scenes/heroes/male_lizard.png"),
            female_elf: asset_server.load("scenes/heroes/female_elf.png"),
            female_knight: asset_server.load("scenes/heroes/female_knight.png"),
            female_wizard: asset_server.load("scenes/heroes/female_wizard.png"),
            female_lizard: asset_server.load("scenes/heroes/female_lizard.png"),
        },
        flag_materials: FlagMaterials {
            vietnam: asset_server.load("scenes/vietnam.png"),
            united_states: asset_server.load("scenes/united_states.png"),
        },
    };

    let ingame_materials: InGameMaterials = InGameMaterials {
        heroes_materials: HeroesMaterials {
            male_elf: asset_server.load("ingame/heroes/male_elf.png"),
            male_knight: asset_server.load("ingame/heroes/male_knight.png"),
            male_wizard: asset_server.load("ingame/heroes/male_wizard.png"),
            male_lizard: asset_server.load("ingame/heroes/male_lizard.png"),
            female_elf: asset_server.load("ingame/heroes/female_elf.png"),
            female_knight: asset_server.load("ingame/heroes/female_knight.png"),
            female_wizard: asset_server.load("ingame/heroes/female_wizard.png"),
            female_lizard: asset_server.load("ingame/heroes/female_lizard.png"),
        },
        weapons_materials: WeaponsMaterials {
            bow: asset_server.load("ingame/weapons/bow.png"),
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
        bullet_materials: BulletsMaterials {
            arrow: asset_server.load("ingame/bullets/arrow.png"),
            bullet: asset_server.load("ingame/bullets/bullet.png"),
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
        hearts_materials: HeartsMaterials {
            full_heart: asset_server.load("ingame/hearts/full_heart.png"),
            half_heart: asset_server.load("ingame/hearts/half_heart.png"),
            empty_heart: asset_server.load("ingame/hearts/empty_heart.png"),
        },
        monsters_materials: MonstersMaterials {
            small_zombie: asset_server.load("ingame/monsters/small_zombie.png"),
            zombie: asset_server.load("ingame/monsters/zombie.png"),
            big_zombie: asset_server.load("ingame/monsters/big_zombie.png"),
            goblin: asset_server.load("ingame/monsters/goblin.png"),
            orc: asset_server.load("ingame/monsters/orc.png"),
            ogre: asset_server.load("ingame/monsters/ogre.png"),
            imp: asset_server.load("ingame/monsters/imp.png"),
            chort: asset_server.load("ingame/monsters/chort.png"),
            big_demon: asset_server.load("ingame/monsters/big_demon.png"),
            swampy: asset_server.load("ingame/monsters/swampy.png"),
        },
        potions_materials: PotionsMaterials {
            heal: asset_server.load("ingame/potions/heal.png"),
            focus: asset_server.load("ingame/potions/focus.png"),
            speed_up: asset_server.load("ingame/potions/speed_up.png"),
            evasion_up: asset_server.load("ingame/potions/evasion_up.png"),
        },
    };

    commands.insert_resource(font_materials);
    commands.insert_resource(scenes_materials);
    commands.insert_resource(ingame_materials);
}

fn load_data(mut commands: Commands) {
    commands.insert_resource(GameData::new());
    commands.insert_resource(Rooms::new());
    commands.insert_resource(PauseFlag::default());
}

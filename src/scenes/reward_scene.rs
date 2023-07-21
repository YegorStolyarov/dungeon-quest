use bevy::prelude::*;
use std::time::Duration;

use crate::components::player::PlayerComponent;
use crate::components::player_list_effects::PlayerListEffectsComponent;
use crate::components::skill::SkillComponent;
use crate::components::weapon::WeaponComponent;
use crate::components::weapon_shoot_attack::WeaponShootAttackComponent;
use crate::components::weapon_swing_attack::WeaponSwingAttackComponent;
use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::game_data::GameData;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::resources::upgrade::upgrade_controller::UpgradeController;
use crate::resources::upgrade::upgrade_type::UpgradeType;
use crate::scenes::SceneState;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 6.0;
const BOX_HEIGHT_TILES: f32 = 3.0;

const BOX_ARRAY: [[i8; 6]; 3] = [[0, 1, 1, 1, 1, 2], [3, 4, 4, 4, 4, 5], [6, 7, 7, 7, 7, 8]];

#[derive(Resource)]
struct RewardSceneData {
    user_interface_root: Entity,
}

#[derive(Component)]
struct RewardCountDownComponent(Timer);

#[derive(Component)]
struct RewardComponent {
    pub upgrade_type: UpgradeType,
    pub is_collected: bool,
}

const REWARDS: [UpgradeType; 4] = [
    UpgradeType::Weapon,
    UpgradeType::Skill,
    UpgradeType::Stats,
    UpgradeType::Effect,
];

pub struct RewardScenePlugin;

impl Plugin for RewardScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::RewardScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::RewardScene)
                .with_system(colddown_handle)
                .with_system(collect_reward),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::RewardScene).with_system(cleanup));
    }
}

fn setup(
    player_dungeon_stats: Res<PlayerDungeonStats>,
    scenes_materials: Res<ScenesMaterials>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let upgrade_type = REWARDS[player_dungeon_stats.current_floor_index - 1].clone();

    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            upgrade_information(parent, &font_materials, &dictionary, upgrade_type);
        })
        .insert(Name::new("RewardUI"))
        .id();

    commands.insert_resource(RewardSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, reward_scene_data: Res<RewardSceneData>) {
    commands
        .entity(reward_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    root.spawn(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
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

                parent.spawn(ImageBundle {
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

fn upgrade_information(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
    upgrade_type: UpgradeType,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let upgrade = glossary.ingame_text.upgrade.clone();

    let value = match upgrade_type {
        UpgradeType::Skill => {
            format!("{} {}", upgrade, glossary.ingame_text.skill)
        }
        UpgradeType::Stats => {
            format!("{} {}", upgrade, glossary.ingame_text.stats)
        }
        UpgradeType::Weapon => {
            format!("{} {}", upgrade, glossary.ingame_text.weapon)
        }
        UpgradeType::Effect => {
            format!("{} {}", upgrade, glossary.ingame_text.effect)
        }
    };

    let width = 300.0;
    let height = 50.0;

    root.spawn(NodeBundle {
        style: Style {
            position: UiRect {
                left: Val::Px(WINDOW_HEIGHT * RESOLUTION / 2.0 - width / 2.0),
                top: Val::Px(WINDOW_HEIGHT / 2.0 - height / 2.0),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size: Size {
                width: Val::Px(width),
                height: Val::Px(height),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                value,
                TextStyle {
                    font: font.clone(),
                    font_size: 35.0,
                    color: Color::DARK_GRAY,
                }
            ).with_alignment(
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            ..Default::default()
        });
    })
    .insert(RewardComponent {
        upgrade_type,
        is_collected: false,
    })
    .insert(Name::new("Reward"))
    .insert(RewardCountDownComponent(Timer::new(
        Duration::from_secs(2),
        TimerMode::Once,
    )));
}

fn colddown_handle(
    mut countdown_query: Query<&mut RewardCountDownComponent>,
    mut state: ResMut<State<SceneState>>,
    time: Res<Time>,
) {
    let mut countdown = countdown_query.single_mut();
    countdown.0.tick(time.delta());
    if countdown.0.finished() {
        state.pop().unwrap();
    }
}

fn collect_reward(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut SkillComponent,
        &mut PlayerListEffectsComponent,
    )>,
    mut weapon_query: Query<(
        &mut WeaponComponent,
        &mut WeaponSwingAttackComponent,
        &mut WeaponShootAttackComponent,
    )>,
    mut reward_query: Query<&mut RewardComponent>,
    upgrade_controller: Res<UpgradeController>,
    game_data: Res<GameData>,
) {
    let mut reward = reward_query.single_mut();

    if !reward.is_collected {
        let (mut player, mut player_skill, mut player_list_effects) = player_query.single_mut();
        let skill_type = player_skill.skill.name.clone();
        let hero_class = player.class.clone();

        match reward.upgrade_type {
            UpgradeType::Weapon => {
                let (mut weapon, mut swing_attack, mut shoot_attack) = weapon_query.single_mut();
                if weapon.level < 3 || (weapon.level < 1 && hero_class == HeroClass::Elf) {
                    let raw_weapons = game_data.get_weapons(hero_class);
                    let raw_weapon = *raw_weapons
                        .iter()
                        .find(|raw_weapon| raw_weapon.level == weapon.level + 1)
                        .expect("Can't find weapon");
                    weapon.upgrade_weapon(&raw_weapon);
                    swing_attack.upgrade(&raw_weapon);
                    shoot_attack.upgrade(&raw_weapon);
                }
            }
            UpgradeType::Stats => {
                player.upgrade_stats(upgrade_controller.get_stats_upgrade());
            }
            UpgradeType::Effect => {
                player_list_effects.upgrade(upgrade_controller.get_effect_upgrade());
            }
            UpgradeType::Skill => {
                player_skill.upgrade(upgrade_controller.get_skill_upgrade(skill_type));
            }
        }
        reward.is_collected = true;
    }
}

use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::slice::Iter;

use crate::components::player::PlayerComponent;
use crate::components::player_list_effects::PlayerListEffectsComponent;
use crate::components::skill::SkillComponent;
use crate::components::weapon::WeaponComponent;
use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::dungeon::wave::Wave;
use crate::resources::game_data::GameData;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::upgrade::upgrade_controller::UpgradeController;
use crate::resources::upgrade::upgrade_type::UpgradeType;
use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::weapon_type::WeaponType;
use crate::scenes::SceneState;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 4.0;
const BOX_HEIGHT_TILES: f32 = 4.0;

const BOX_ARRAY: [[i8; 4]; 4] = [[0, 1, 1, 2], [3, 4, 4, 5], [3, 4, 4, 5], [6, 7, 7, 8]];

#[derive(Component, Copy, Clone, PartialEq, Eq)]
enum RewardsSceneButton {
    One,
    Two,
    Three,
}

#[derive(Component)]
struct Reward {
    upgrade_type: UpgradeType,
}

impl RewardsSceneButton {
    pub fn iterator() -> Iter<'static, RewardsSceneButton> {
        static BUTTONS: [RewardsSceneButton; 3] = [
            RewardsSceneButton::One,
            RewardsSceneButton::Two,
            RewardsSceneButton::Three,
        ];
        BUTTONS.iter()
    }
}

struct RewardsSceneData {
    user_interface_root: Entity,
}

pub struct RewardsScenePlugin;

impl Plugin for RewardsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::RewardsScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::RewardsScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::RewardsScene).with_system(cleanup));
    }
}

fn setup(
    upgrade_controller: Res<UpgradeController>,
    scenes_materials: Res<ScenesMaterials>,
    weapon_query: Query<&WeaponComponent>,
    player_query: Query<&PlayerComponent>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let player = player_query.single();
    let hero_class = player.class.clone();
    let weapon_component = weapon_query.single();
    let three_upgrades = upgrade_controller.get_three_upgrades(hero_class, weapon_component.level);

    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            buttons(parent, &font_materials, &dictionary, three_upgrades);
        })
        .insert(Name::new("RewardsUI"))
        .id();

    commands.insert_resource(RewardsSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, rewards_scene_data: Res<RewardsSceneData>) {
    commands
        .entity(rewards_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (row_index, row) in BOX_ARRAY.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                let position: Rect<Val> = Rect {
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

fn buttons(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
    three_upgrades: Vec<UpgradeType>,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|grandparent| {
        for (index, button) in RewardsSceneButton::iterator().enumerate() {
            let upgrade_type = three_upgrades[index].clone();

            let value = match upgrade_type {
                UpgradeType::Weapon => glossary.ingame_text.weapon.clone(),
                UpgradeType::Stats => glossary.ingame_text.stats.clone(),
                UpgradeType::Skill => glossary.ingame_text.skill.clone(),
                UpgradeType::Effect => glossary.ingame_text.effect.clone(),
            };

            let top_position = match *button {
                RewardsSceneButton::One => 220.0,
                RewardsSceneButton::Two => 270.0,
                RewardsSceneButton::Three => 320.0,
            };

            grandparent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px(435.0),
                            top: Val::Px(top_position),
                            right: Val::Auto,
                            bottom: Val::Auto,
                        },
                        size: Size {
                            width: Val::Px(150.0),
                            height: Val::Px(35.0),
                        },
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            value.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 35.0,
                                color: Color::GRAY,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(Reward { upgrade_type })
                .insert(Name::new(value.clone()))
                .insert(button.clone());
        }
    })
    .insert(Name::new("Rewards"));
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &Reward, &Children),
        (Changed<Interaction>, With<RewardsSceneButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut SkillComponent,
        &mut PlayerListEffectsComponent,
    )>,
    mut weapon_query: Query<(&mut WeaponComponent, &mut Sprite, &mut Handle<Image>)>,
    upgrade_controller: Res<UpgradeController>,
    ingame_materials: Res<InGameMaterials>,
    mut state: ResMut<State<SceneState>>,
    mut wave: ResMut<Wave>,
    game_data: Res<GameData>,
) {
    for (interaction, reward, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK.into(),
            Interaction::Clicked => {
                let (mut player, mut player_skill, mut player_list_effects) =
                    player_query.single_mut();
                let hero_class = player.class.clone();

                match reward.upgrade_type {
                    UpgradeType::Weapon => {
                        let (mut weapon_component, mut weapon_sprite, mut weapon_texture) =
                            weapon_query.single_mut();
                        if weapon_component.level < 3
                            || (weapon_component.level < 1 && hero_class == HeroClass::Elf)
                        {
                            let weapons = game_data.get_weapons(hero_class);

                            let weapon = weapons
                                .iter()
                                .find(|weapon| weapon.level == weapon_component.level + 1)
                                .expect("Can't find weapon")
                                .clone();

                            weapon_component.upgrade_weapon(weapon);

                            weapon_sprite.custom_size = Some(Vec2::new(
                                weapon_component.size_width * weapon_component.scale,
                                weapon_component.size_height * weapon_component.scale,
                            ));

                            weapon_sprite.anchor = match weapon_component.attack_type {
                                AttackType::Swing => Anchor::BottomCenter,
                                AttackType::Throw => Anchor::BottomCenter,
                                AttackType::Shoot => Anchor::Center,
                            };

                            *weapon_texture = match weapon_component.name {
                                WeaponType::ShortSword => {
                                    ingame_materials.weapons_materials.short_sword.clone()
                                }
                                WeaponType::Sword => {
                                    ingame_materials.weapons_materials.sword.clone()
                                }
                                WeaponType::BigMachete => {
                                    ingame_materials.weapons_materials.machete.clone()
                                }
                                WeaponType::SmallWand => {
                                    ingame_materials.weapons_materials.small_wand.clone()
                                }
                                WeaponType::MagicWand => {
                                    ingame_materials.weapons_materials.magic_wand.clone()
                                }
                                WeaponType::MagicSword => {
                                    ingame_materials.weapons_materials.magic_sword.clone()
                                }
                                WeaponType::Mace => ingame_materials.weapons_materials.mace.clone(),
                                WeaponType::BigHammer => {
                                    ingame_materials.weapons_materials.big_hammer.clone()
                                }
                                WeaponType::SmallHammer => {
                                    ingame_materials.weapons_materials.small_hammer.clone()
                                }
                                WeaponType::Bow => ingame_materials.weapons_materials.bow.clone(),
                                WeaponType::Spear => {
                                    ingame_materials.weapons_materials.spear.clone()
                                }
                            };
                        }
                    }
                    UpgradeType::Stats => {
                        player.upgrade_stats(upgrade_controller.get_stats_upgrade());
                    }
                    UpgradeType::Effect => {
                        player_list_effects.upgrade(upgrade_controller.get_effect_upgrade());
                    }
                    UpgradeType::Skill => {
                        let skill_type = player_skill.skill.name.clone();
                        player_skill.upgrade(upgrade_controller.get_skill_upgrade(skill_type));
                    }
                }
                wave.next_wave();
                state.pop().unwrap();
            }
        }
    }
}

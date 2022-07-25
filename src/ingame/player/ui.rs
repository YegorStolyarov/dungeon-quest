use bevy::prelude::*;
use std::slice::Iter;

use crate::config::{RESOLUTION, WINDOW_HEIGHT};
use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::skill::skill_type::SkillType;
use crate::materials::Materials;
use crate::resources::dictionary::Dictionary;

#[derive(Component, Clone)]
pub enum InformationText {
    Strength,
    Intelligence,
    MovementSpeed,
    CriticalChance,
    DodgeChance,
    RestoreChance,
    DamagePercentBonus,
}

#[derive(Component)]
pub struct Heart {
    index: f32,
}

#[derive(Component)]
pub struct Hearts;

#[derive(Component)]
pub struct SkillDuration;

#[derive(Component)]
pub struct SkilLCooldown;

impl InformationText {
    pub fn iterator() -> Iter<'static, InformationText> {
        static TEXTS: [InformationText; 7] = [
            InformationText::Strength,
            InformationText::Intelligence,
            InformationText::MovementSpeed,
            InformationText::CriticalChance,
            InformationText::DodgeChance,
            InformationText::RestoreChance,
            InformationText::DamagePercentBonus,
        ];
        TEXTS.iter()
    }
}

pub struct PlayerUIData {
    user_interface_root: Entity,
}

pub fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    ingame_materials: Res<InGameMaterials>,
    dictionary: Res<Dictionary>,
) {
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
            hearts(parent, &ingame_materials);
            information_texts(parent, &materials, &dictionary);
            skill_cooldown(parent);
            skill_duration(parent);
        })
        .insert(Name::new("PlayerUI"))
        .id();

    commands.insert_resource(PlayerUIData {
        user_interface_root,
    });
}

pub fn cleanup(mut commands: Commands, player_ui_data: Res<PlayerUIData>) {
    commands
        .entity(player_ui_data.user_interface_root)
        .despawn_recursive();
}

pub fn information_texts(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let font_size = 20.0;

    let glossary = dictionary.get_glossary();
    let ingame_gloassary = glossary.ingame_text;

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (index, information_text) in InformationText::iterator().enumerate() {
            let left_position = 10.0;
            let top_position = 430.0 + index as f32 * font_size;

            let component_name = match *information_text {
                InformationText::Strength => ingame_gloassary.strength.clone(),
                InformationText::Intelligence => ingame_gloassary.intelligence.clone(),
                InformationText::MovementSpeed => ingame_gloassary.movement_speed.clone(),
                InformationText::CriticalChance => ingame_gloassary.critical_chance.clone(),
                InformationText::DodgeChance => ingame_gloassary.dodge_chance.clone(),
                InformationText::RestoreChance => ingame_gloassary.restore_chance.clone(),
                InformationText::DamagePercentBonus => {
                    ingame_gloassary.damage_percent_bonus.clone()
                }
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
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Name::new(component_name))
                .insert(information_text.clone());
        }
    })
    .insert(Name::new("Information"));
}

pub fn information_texts_handle(
    mut information_texts_query: Query<(&InformationText, &mut Text), Without<Player>>,
    player_query: Query<&Player>,
    dictionary: Res<Dictionary>,
) {
    let glossary = dictionary.get_glossary();
    let ingame_gloassary = glossary.ingame_text;
    let player = player_query.single();

    for (information_text, mut text) in information_texts_query.iter_mut() {
        match *information_text {
            InformationText::Strength => {
                text.sections[0].value =
                    format!("{}: {}", ingame_gloassary.strength.clone(), player.strength);
            }
            InformationText::Intelligence => {
                text.sections[0].value = format!(
                    "{}: {}",
                    ingame_gloassary.intelligence.clone(),
                    player.intelligence
                );
            }
            InformationText::MovementSpeed => {
                text.sections[0].value = format!(
                    "{}: {}",
                    ingame_gloassary.movement_speed.clone(),
                    player.speed
                );
            }
            InformationText::CriticalChance => {
                text.sections[0].value = format!(
                    "{}: {}%",
                    ingame_gloassary.critical_chance.clone(),
                    (player.critical_chance * 100.0) as usize
                );
            }
            InformationText::DodgeChance => {
                text.sections[0].value = format!(
                    "{}: {}%",
                    ingame_gloassary.dodge_chance.clone(),
                    (player.dodge_chance * 100.0) as usize
                );
            }
            InformationText::RestoreChance => {
                text.sections[0].value = format!(
                    "{}: {}%",
                    ingame_gloassary.restore_chance.clone(),
                    (player.restore_chance * 100.0) as usize
                );
            }
            InformationText::DamagePercentBonus => {
                text.sections[0].value = format!(
                    "{}: {}%",
                    ingame_gloassary.damage_percent_bonus.clone(),
                    (player.damage_percent_bonus * 100.0) as usize
                );
            }
        }
    }
}

fn hearts(root: &mut ChildBuilder, ingame_materials: &InGameMaterials) {
    root.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(30.0 * 5.0), Val::Px(30.0 * 2.0)),
            position: Rect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        for row_index in 0..=1 {
            for column_index in 0..=4 {
                let left_position = 32.0 * column_index as f32;
                let top_position = 32.0 * row_index as f32;

                let heart = Heart {
                    index: (row_index * 5 + column_index + 1) as f32,
                };

                let index = heart.index;

                parent
                    .spawn_bundle(ImageBundle {
                        calculated_size: CalculatedSize {
                            size: Size {
                                width: 16.0,
                                height: 16.0,
                            },
                        },
                        style: Style {
                            size: Size::new(Val::Px(35.0), Val::Px(35.0)),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                left: Val::Px(left_position),
                                top: Val::Px(top_position),
                                bottom: Val::Auto,
                                right: Val::Auto,
                            },
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        image: UiImage(ingame_materials.hearts_materials.empty_heart.clone()),
                        ..Default::default()
                    })
                    .insert(heart)
                    .insert(Name::new(format!("Heart:{}", index)));
            }
        }
    })
    .insert(Hearts)
    .insert(Name::new("Hearts"));
}

pub fn hearts_handle(
    mut heart_query: Query<(&Heart, &mut Visibility, &mut UiImage)>,
    ingame_materials: Res<InGameMaterials>,
    player_query: Query<&Player>,
) {
    let player = player_query.single();

    let current_health_points = player.current_health_points;
    let current_health_points_floor = current_health_points.floor();
    let max_health_points = player.max_health_points;

    for (heart, mut visibility, mut ui_image) in heart_query.iter_mut() {
        if heart.index <= max_health_points {
            visibility.is_visible = true;
        }

        ui_image.0 = ingame_materials.hearts_materials.empty_heart.clone();

        if heart.index <= current_health_points_floor {
            ui_image.0 = ingame_materials.hearts_materials.full_heart.clone();
        }

        if current_health_points_floor < current_health_points {
            if heart.index == current_health_points_floor + 1.0 {
                ui_image.0 = ingame_materials.hearts_materials.half_heart.clone();
            }
        }
    }
}

pub fn skill_duration(root: &mut ChildBuilder) {
    let length = 300.0;

    let size = Size::new(Val::Px(length), Val::Px(10.0));

    root.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(WINDOW_HEIGHT * RESOLUTION / 2.0 - length / 2.0),
                ..Default::default()
            },
            size,
            ..Default::default()
        },
        color: UiColor(Color::ORANGE),
        visibility: Visibility { is_visible: false },
        ..Default::default()
    })
    .insert(SkillDuration);
}

pub fn skill_duration_handle(
    mut skill_duration_query: Query<(&mut Style, &mut Visibility), With<SkillDuration>>,
    player_skill: Res<PlayerSkill>,
) {
    let max_length = 300.0;
    let (mut style, mut visibility) = skill_duration_query.single_mut();
    if !player_skill.duration.finished() {
        visibility.is_visible = true;
        let percent_left = player_skill.duration.percent_left();
        let length = max_length * percent_left;
        style.position.left = Val::Px(WINDOW_HEIGHT * RESOLUTION / 2.0 - length / 2.0);
        style.size.width = Val::Px(length);
    } else {
        visibility.is_visible = false;
        style.size.width = Val::Px(max_length);
    }
}

pub fn skill_cooldown(root: &mut ChildBuilder) {
    let length = 250.0;
    let size = Size::new(Val::Px(10.0), Val::Px(length));

    root.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(WINDOW_HEIGHT / 2.0 - length / 2.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size,
            ..Default::default()
        },
        color: UiColor(Color::GREEN),
        visibility: Visibility { is_visible: true },
        ..Default::default()
    })
    .insert(SkilLCooldown);
}

pub fn skill_cooldown_handle(
    mut skill_cooldown_query: Query<(&mut Style, &mut Visibility), With<SkilLCooldown>>,
    player_skill: Res<PlayerSkill>,
) {
    let max_length = 250.0;
    let (mut style, mut visibility) = skill_cooldown_query.single_mut();
    if player_skill.skill.name == SkillType::Armor {
        let require_monsters = player_skill.require_monsters as f32;
        let monster_counter = player_skill.monster_counter as f32;
        if monster_counter <= require_monsters {
            let percent = monster_counter / require_monsters;
            let length = max_length * percent;
            style.size.height = Val::Px(length);
        }
    } else {
        if !player_skill.cooldown.finished() {
            visibility.is_visible = true;
            let percent_left = player_skill.cooldown.percent_left();
            let length = max_length * percent_left;
            style.size.height = Val::Px(length);
        } else {
            visibility.is_visible = false;
            style.size.height = Val::Px(max_length);
        }
    }
}

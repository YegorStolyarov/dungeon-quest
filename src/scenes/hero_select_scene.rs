use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::ingame::InGameMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::game_mode::GameMode;
use crate::resources::hero::gender::Gender;
use crate::resources::hero::hero_class::HeroClass;
use crate::resources::profile::Profile;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIZE: f32 = 50.0;
const BOX_TILE_SIZE: f32 = 60.0;

const BOX_ARRAY: [[i8; 13]; 9] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(Component)]
struct ReturnButtonComponent;

#[derive(Component, PartialEq, Clone)]
pub enum ButtonComponent {
    MaleElf,
    FemaleElf,
    MaleKnight,
    FemaleKnight,
    MaleWizard,
    FemaleWizard,
    MaleLizard,
    FemaleLizard,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::MaleElf,
            ButtonComponent::MaleKnight,
            ButtonComponent::MaleWizard,
            ButtonComponent::MaleLizard,
            ButtonComponent::FemaleElf,
            ButtonComponent::FemaleKnight,
            ButtonComponent::FemaleWizard,
            ButtonComponent::FemaleLizard,
        ]
        .iter()
    }
}

struct AnimationController {
    run_animation: bool,
    hero_image: HeroImageComponent,
    timer: Timer,
}

pub struct HeroSelectScenePlugin;

#[derive(Component, Clone, PartialEq, Eq)]
enum HeroImageComponent {
    MaleElf,
    FemaleElf,
    MaleKnight,
    FemaleKnight,
    MaleWizard,
    FemaleWizard,
    MaleLizard,
    FemaleLizard,
}

struct HeroSelectSceneData {
    sprite_bundle: Entity,
    user_interface_root: Entity,
}

impl Plugin for HeroSelectScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::HeroSelectScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::HeroSelectScene)
                .with_system(return_button_handle)
                .with_system(hero_select_handle)
                .with_system(hero_image_animation_handle),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::HeroSelectScene).with_system(cleanup));
    }
}

fn setup(
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    ingame_materials: Res<InGameMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    let sprite_bundle = commands
        .spawn_bundle(SpriteBundle {
            texture: scenes_materials.sub_background_image.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            heros_images(parent, &ingame_materials, texture_atlases)
        })
        .insert(Name::new("SpriteBundle"))
        .id();

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
            select_hero_text(parent, &font_materials, &dictionary);
            return_button(parent, &scenes_materials);
            heros_buttons(parent);
        })
        .insert(Name::new("UIRoot"))
        .id();

    commands.insert_resource(HeroSelectSceneData {
        sprite_bundle,
        user_interface_root,
    });

    commands.insert_resource(AnimationController {
        run_animation: false,
        hero_image: HeroImageComponent::MaleElf,
        timer: Timer::from_seconds(0.1, true),
    });
}

fn cleanup(mut commands: Commands, hero_select_scene_data: Res<HeroSelectSceneData>) {
    commands
        .entity(hero_select_scene_data.user_interface_root)
        .despawn_recursive();

    commands
        .entity(hero_select_scene_data.sprite_bundle)
        .despawn_recursive();

    commands.remove_resource::<AnimationController>();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let start_x = -340.0;
    let start_y = 230.0;

    for (row_index, row) in BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
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

            root.spawn_bundle(SpriteBundle {
                texture: image,
                transform: Transform {
                    translation: Vec3::new(
                        start_x + 60.0 * column_index as f32,
                        start_y - 60.0 * row_index as f32,
                        0.0,
                    ),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BOX_TILE_SIZE, BOX_TILE_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn return_button(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
    let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();
    root.spawn_bundle(ButtonBundle {
        style: Style {
            position: Rect {
                left: Val::Px(RETURN_BUTTON_SIZE / 2.0),
                top: Val::Px(RETURN_BUTTON_SIZE / 2.0),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size: Size {
                width: Val::Px(RETURN_BUTTON_SIZE),
                height: Val::Px(RETURN_BUTTON_SIZE),
            },
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image: UiImage(handle_image),
        ..Default::default()
    })
    .insert(Name::new("ReturnButton"))
    .insert(ReturnButtonComponent);
}

fn return_button_handle(
    mut button_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<ReturnButtonComponent>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, mut ui_image) in button_query.iter_mut() {
        match *interaction {
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
        }
    }
}

fn heros_images(
    root: &mut ChildBuilder,
    ingame_materials: &InGameMaterials,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut index = 0;
    let hero_image_positions: [[f32; 2]; 8] = [
        [-250.0, 75.0],
        [-250.0, -100.0],
        [-75.0, 75.0],
        [-75.0, -100.0],
        [100.0, 75.0],
        [100.0, -100.0],
        [275.0, 75.0],
        [275.0, -100.0],
    ];

    for hero_class in HeroClass::iterator() {
        for gender in Gender::iterator() {
            let hero_tileset;
            let hero_image;
            let component_name;

            match hero_class {
                HeroClass::Elf => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heros_materials.male_elf.clone();
                        hero_image = HeroImageComponent::MaleElf;
                        component_name = format!("{}_{}", "Elf", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heros_materials.female_elf.clone();
                        hero_image = HeroImageComponent::FemaleElf;
                        component_name = format!("{}_{}", "Elf", "Female");
                    }
                },
                HeroClass::Knight => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heros_materials.male_knight.clone();
                        hero_image = HeroImageComponent::MaleKnight;
                        component_name = format!("{}_{}", "Knight", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heros_materials.female_knight.clone();
                        hero_image = HeroImageComponent::FemaleKnight;
                        component_name = format!("{}_{}", "Knight", "Female");
                    }
                },
                HeroClass::Lizard => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heros_materials.male_lizard.clone();
                        hero_image = HeroImageComponent::MaleLizard;
                        component_name = format!("{}_{}", "Lizard", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heros_materials.female_lizard.clone();
                        hero_image = HeroImageComponent::FemaleLizard;
                        component_name = format!("{}_{}", "Lizard", "Female");
                    }
                },
                HeroClass::Wizard => match gender {
                    Gender::Male => {
                        hero_tileset = ingame_materials.heros_materials.male_wizard.clone();
                        hero_image = HeroImageComponent::MaleWizard;
                        component_name = format!("{}_{}", "Wizard", "Male");
                    }
                    Gender::Female => {
                        hero_tileset = ingame_materials.heros_materials.female_wizard.clone();
                        hero_image = HeroImageComponent::FemaleWizard;
                        component_name = format!("{}_{}", "Wizard", "Female");
                    }
                },
            };

            let texture_atlas = TextureAtlas::from_grid(hero_tileset, Vec2::new(16.0, 28.0), 9, 1);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let x = hero_image_positions[index][0];
            let y = hero_image_positions[index][1];

            root.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    scale: Vec3::splat(6.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(component_name))
            .insert(hero_image);
            index += 1;
        }
    }
}

fn select_hero_text(
    root: &mut ChildBuilder,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(390.0),
                top: Val::Px(95.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            glossary.shared_text.select_hero,
            TextStyle {
                font: font,
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
    .insert(Name::new("SelectHeroText"));
}

fn heros_buttons(root: &mut ChildBuilder) {
    let button_positions: [[f32; 2]; 8] = [
        [210.0, 170.0],
        [380.0, 170.0],
        [560.0, 170.0],
        [740.0, 170.0],
        [210.0, 350.0],
        [380.0, 350.0],
        [560.0, 350.0],
        [740.0, 350.0],
    ];

    for (index, value) in ButtonComponent::iterator().enumerate() {
        let position = Rect {
            left: Val::Px(button_positions[index][0]),
            top: Val::Px(button_positions[index][1]),
            right: Val::Auto,
            bottom: Val::Auto,
        };

        let component_name = match index {
            1 => "MaleElf",
            2 => "MaleKnight",
            3 => "MaleLizard",
            4 => "MaleWizard",
            5 => "FemaleElf",
            6 => "FemaleKnight",
            7 => "FemaleLizard",
            _ => "FemaleWizard",
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position,
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(150.0),
                },
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new(component_name))
        .insert(value.clone());
    }
}

fn hero_select_handle(
    mut button_query: Query<(&Interaction, &ButtonComponent), (Changed<Interaction>, With<Button>)>,
    mut profile: ResMut<Profile>,
    mut animation_controller: ResMut<AnimationController>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button) in button_query.iter_mut() {
        match interaction {
            Interaction::None => animation_controller.run_animation = false,
            Interaction::Hovered => {
                animation_controller.run_animation = true;
                match button {
                    ButtonComponent::MaleElf => {
                        animation_controller.hero_image = HeroImageComponent::MaleElf
                    }
                    ButtonComponent::FemaleElf => {
                        animation_controller.hero_image = HeroImageComponent::FemaleElf
                    }
                    ButtonComponent::MaleKnight => {
                        animation_controller.hero_image = HeroImageComponent::MaleKnight
                    }
                    ButtonComponent::FemaleKnight => {
                        animation_controller.hero_image = HeroImageComponent::FemaleKnight
                    }
                    ButtonComponent::MaleLizard => {
                        animation_controller.hero_image = HeroImageComponent::MaleLizard
                    }
                    ButtonComponent::FemaleLizard => {
                        animation_controller.hero_image = HeroImageComponent::FemaleLizard
                    }
                    ButtonComponent::MaleWizard => {
                        animation_controller.hero_image = HeroImageComponent::MaleWizard
                    }
                    ButtonComponent::FemaleWizard => {
                        animation_controller.hero_image = HeroImageComponent::FemaleWizard
                    }
                };
            }
            Interaction::Clicked => {
                profile.set_hero(button.clone());
                if profile.game_mode == GameMode::ClassicMode {
                    state
                        .set(SceneState::PreClassicMode)
                        .expect("Couldn't switch state to Pre Classic Mode");
                } else {
                    state
                        .set(SceneState::PreSurvivalMode)
                        .expect("Couldn't switch state to Pre Survival Mode");
                }
            }
        }
    }
}

fn hero_image_animation_handle(
    time: Res<Time>,
    mut query: Query<(&HeroImageComponent, &mut TextureAtlasSprite)>,
    mut animation_controller: ResMut<AnimationController>,
) {
    for (hero_image, mut sprite) in query.iter_mut() {
        if animation_controller.run_animation && *hero_image == animation_controller.hero_image {
            animation_controller.timer.tick(time.delta());
            if animation_controller.timer.just_finished() {
                let min_index = 0;
                let max_index = 3;
                if sprite.index > max_index || sprite.index < min_index {
                    sprite.index = min_index;
                } else {
                    sprite.index += 1;
                }
            }
        }
    }
}

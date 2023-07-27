use bevy::app::AppExit;
use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::scenes::SceneState;

const MAIN_MENU_BOX_ARRAY: [[i8; 5]; 8] = [
    [0, 1, 1, 1, 2],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [6, 7, 7, 7, 8],
];
const FONT_SIZE: f32 = 36.0;
const MAIN_MENU_BOX_TILE_SIZE: f32 = 50.0;

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Play,
    Highscore,
    Options,
    Help,
    Credits,
    Quit,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Play,
            ButtonComponent::Highscore,
            ButtonComponent::Options,
            ButtonComponent::Help,
            ButtonComponent::Credits,
            ButtonComponent::Quit,
        ]
        .iter()
    }
}

#[derive(Resource)]
struct MainMenuSceneData {
    user_interface_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::MainMenuScene), setup);
        app.add_systems(Update,button_handle_system.run_if(in_state(SceneState::MainMenuScene)));
        app.add_systems(OnExit(SceneState::MainMenuScene), cleanup);
    }
}

fn setup(
    scenes_materials: Res<ScenesMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
) {
    let user_interface_root = commands
        .spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            image: UiImage::new(scenes_materials.main_background_image.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            main_menu_box(parent, &scenes_materials.menu_box_materials);
            buttons(parent, &font_materials, dictionary);
        })
        .id();

    commands.insert_resource(MainMenuSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands
        .entity(main_menu_scene_data.user_interface_root)
        .despawn_recursive();
}

fn main_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {

    for (row_index, row) in MAIN_MENU_BOX_ARRAY.iter().enumerate() {
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

            root.spawn(ImageBundle {
                image: UiImage::new(image),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * column_index as f32),
                    top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * row_index as f32),
                    bottom: Val::Auto,
                    right: Val::Auto,
                    width: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
                    height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
                    ..Default::default()
                },

                ..Default::default()
            });
        }
    }
}

fn buttons(root: &mut ChildBuilder, materials: &Res<FontMaterials>, dictionary: Res<Dictionary>) {
    let glossary = dictionary.get_glossary();

    for (index, button) in ButtonComponent::iterator().enumerate() {

        root.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(MAIN_MENU_BOX_TILE_SIZE * 3.0),
                height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * (3.0 - 1.0) / 2.0),
                right: Val::Auto,
                top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * (index as f32 + 1.0)),
                bottom: Val::Auto,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            let text: &str = match button {
                ButtonComponent::Play => glossary.main_menu_scene_text.play.as_str(),
                ButtonComponent::Highscore => glossary.main_menu_scene_text.highscore.as_str(),
                ButtonComponent::Options => glossary.main_menu_scene_text.options.as_str(),
                ButtonComponent::Help => glossary.main_menu_scene_text.help.as_str(),
                ButtonComponent::Credits => glossary.main_menu_scene_text.credits.as_str(),
                ButtonComponent::Quit => glossary.main_menu_scene_text.quit.as_str(),
            };

            parent.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: materials.get_font(dictionary.get_current_language()),
                        font_size: FONT_SIZE,
                        color: Color::GRAY,
                    }
                ).with_alignment(
                    TextAlignment::Center
                ),
                ..Default::default()
            });
        })
        .insert(button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<NextState<SceneState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK,
            Interaction::Pressed => {
                text.sections[0].style.color = Color::RED;
                match button {
                    ButtonComponent::Play => state
                        .set(SceneState::GameModeSelectScene),
                    ButtonComponent::Highscore => state
                        .set(SceneState::HighscoreScene),
                    ButtonComponent::Options => state
                        .set(SceneState::OptionsScene),
                    ButtonComponent::Help => state
                        .set(SceneState::HelpScene),
                    ButtonComponent::Credits => state
                        .set(SceneState::CreditsScene),
                    ButtonComponent::Quit => exit.send(AppExit),
                }
            }
        }
    }
}

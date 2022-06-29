use bevy::app::AppExit;
use bevy::prelude::*;
use std::slice::Iter;

use crate::resources::dictionary::Dictionary;
use crate::resources::materials::{scenes::main_menu_scene::MainMenuBoxMaterials, GlobalMaterials};
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
enum MainMenuSceneButton {
    Play,
    Highscore,
    Options,
    Help,
    Credits,
    Quit,
}

impl MainMenuSceneButton {
    pub fn iterator() -> Iter<'static, MainMenuSceneButton> {
        static MAIN_MENU_SCENE_BUTTONS: [MainMenuSceneButton; 6] = [
            MainMenuSceneButton::Play,
            MainMenuSceneButton::Highscore,
            MainMenuSceneButton::Options,
            MainMenuSceneButton::Help,
            MainMenuSceneButton::Credits,
            MainMenuSceneButton::Quit,
        ];
        MAIN_MENU_SCENE_BUTTONS.iter()
    }
}

struct MainMenuSceneData {
    ui_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::MainMenuScene).with_system(setup));
        app.add_system_set(SystemSet::on_exit(SceneState::MainMenuScene).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(SceneState::MainMenuScene).with_system(button_handle_system),
        );
    }
}

fn setup(
    mut commands: Commands,
    global_materials: Res<GlobalMaterials>,
    dictionary: Res<Dictionary>,
) {
    let ui_root = commands
        .spawn_bundle(root(&global_materials))
        .with_children(|parent| {
            main_menu_box(
                parent,
                &global_materials
                    .main_menu_scene_materials
                    .main_menu_box_materials,
            );
            buttons(parent, &global_materials, dictionary);
        })
        .id();

    commands.insert_resource(MainMenuSceneData { ui_root });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands
        .entity(main_menu_scene_data.ui_root)
        .despawn_recursive();
}

fn root(global_materials: &Res<GlobalMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(global_materials.main_menu_background.clone()),
        ..Default::default()
    }
}

fn main_menu_box(root: &mut ChildBuilder, main_menu_box_materials: &MainMenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
        height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
    };

    for (row_index, row) in MAIN_MENU_BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: Rect<Val> = Rect {
                left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * row_index as f32),
                bottom: Val::Auto,
                right: Val::Auto,
            };

            let image: Handle<Image> = match value {
                0 => main_menu_box_materials.top_right.clone(),
                1 => main_menu_box_materials.top_center.clone(),
                2 => main_menu_box_materials.top_left.clone(),
                3 => main_menu_box_materials.mid_right.clone(),
                4 => main_menu_box_materials.mid_center.clone(),
                5 => main_menu_box_materials.mid_left.clone(),
                6 => main_menu_box_materials.bottom_right.clone(),
                7 => main_menu_box_materials.bottom_center.clone(),
                8 => main_menu_box_materials.bottom_left.clone(),
                _ => panic!("Unknown resources"),
            };

            root.spawn_bundle(NodeBundle {
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
}

fn buttons(
    root: &mut ChildBuilder,
    global_materials: &Res<GlobalMaterials>,
    dictionary: Res<Dictionary>,
) {
    let glossary = dictionary.get_glossary();

    for (index, button) in MainMenuSceneButton::iterator().enumerate() {
        let position: Rect<Val> = Rect {
            left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * (3.0 - 1.0) / 2.0),
            right: Val::Auto,
            top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * (index as f32 + 1.0)),
            bottom: Val::Auto,
        };

        let size = Size {
            width: Val::Px(MAIN_MENU_BOX_TILE_SIZE * 3.0),
            height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                position,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            let text: &str = match button {
                MainMenuSceneButton::Play => glossary.main_menu_scene_text.play.as_str(),
                MainMenuSceneButton::Highscore => glossary.main_menu_scene_text.highscore.as_str(),
                MainMenuSceneButton::Options => glossary.main_menu_scene_text.options.as_str(),
                MainMenuSceneButton::Help => glossary.main_menu_scene_text.help.as_str(),
                MainMenuSceneButton::Credits => glossary.main_menu_scene_text.credits.as_str(),
                MainMenuSceneButton::Quit => glossary.main_menu_scene_text.quit.as_str(),
            };

            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: global_materials.get_font(dictionary.get_current_language()),
                        font_size: FONT_SIZE,
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
        .insert(button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &MainMenuSceneButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<SceneState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK.into(),
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED.into();
                match button {
                    MainMenuSceneButton::Play => state
                        .set(SceneState::GameModeSelectScene)
                        .expect("Couldn't switch state to Loading Screen"),
                    MainMenuSceneButton::Highscore => state
                        .set(SceneState::HighscoreScene)
                        .expect("Couldn't switch state to Highscore Scene"),
                    MainMenuSceneButton::Options => state
                        .set(SceneState::OptionsScene)
                        .expect("Couldn't switch state to Options Scene"),
                    MainMenuSceneButton::Help => state
                        .set(SceneState::HelpScene)
                        .expect("Couldn't switch state to Help Scene"),
                    MainMenuSceneButton::Credits => state
                        .set(SceneState::CreditsScene)
                        .expect("Couldn't switch state to Credits Scene"),
                    MainMenuSceneButton::Quit => exit.send(AppExit),
                }
            }
        }
    }
}

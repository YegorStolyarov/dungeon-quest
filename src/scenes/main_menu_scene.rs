use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::egui::Ui;

use crate::config::*;
use crate::scenes::ApplicationScene;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;
const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

const BUTTON_POSITIONS: [[f32; 2]; 1] = [
    [SEPARATE, WINDOW_HEIGHT / 2.0], // Play
];

const FONT_SIZE: f32 = 30.0;

#[derive(Component)]
pub enum MainMenuSceneButton {
    Play,
}

struct MainMenuSceneData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationScene::MainMenuScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_exit(ApplicationScene::MainMenuScene).with_system(cleanup),
        );
        app.add_system_set(
            SystemSet::on_update(ApplicationScene::MainMenuScene).with_system(button_handle_system),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&asset_server))
        .with_children(|parent| {
            parent
                .spawn_bundle(button_bundle(MainMenuSceneButton::Play, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuSceneButton::Play, &asset_server));
                })
                .insert(MainMenuSceneButton::Play);
        })
        .id();

    commands.insert_resource(MainMenuSceneData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands
        .entity(main_menu_scene_data.ui_root)
        .despawn_recursive();
    commands
        .entity(main_menu_scene_data.camera_entity)
        .despawn_recursive();
}

fn root(asset_server: &Res<AssetServer>) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(asset_server.load(MENU_BACKGROUND_IMAGE)),
        ..Default::default()
    }
}

fn button_bundle(
    main_menu_scene_button: MainMenuSceneButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT));

    let possition: [f32; 2] = match main_menu_scene_button {
        MainMenuSceneButton::Play => BUTTON_POSITIONS[0],
    };

    ButtonBundle {
        style: Style {
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(possition[0]),
                top: Val::Px(possition[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load("images/panel_Example1.png")),
        ..Default::default()
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &MainMenuSceneButton, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<ApplicationScene>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, mut color, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::GREEN.into();
            }
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED.into();
                match button {
                    MainMenuSceneButton::Play => state
                        .set(ApplicationScene::LoadingScene)
                        .expect("Couldn't switch state to Loading Screen"),
                }
            }
        }
    }
}

fn text_bundle(
    main_menu_scene_button: MainMenuSceneButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match main_menu_scene_button {
        MainMenuSceneButton::Play => "PLAY",
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/DungeonFont.ttf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    }
}

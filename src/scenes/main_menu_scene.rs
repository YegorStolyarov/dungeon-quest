use bevy::{app::AppExit, prelude::*};

use crate::config::*;
use crate::resources::scene::ApplicationScene;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;
const SEPARATE: f32 = BUTTON_HEIGHT / 4.0;

const BUTTON_POSITIONS: [[f32; 2]; 4] = [
    [SEPARATE, SEPARATE],                             // Play
    [SEPARATE, SEPARATE * 2.0 + BUTTON_HEIGHT],       // Demos
    [SEPARATE, SEPARATE * 3.0 + BUTTON_HEIGHT * 2.0], // Setting
    [SEPARATE, SEPARATE * 4.0 + BUTTON_HEIGHT * 3.0], // Quit
];

const FONT_SIZE: f32 = 80.0;

#[derive(Component)]
pub enum MainMenuSceneButton {
    Play,
    Demos,
    Setting,
    Quit,
}

struct MainMenuSceneData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationScene::MainMenuScene).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationScene::MainMenuScene).with_system(cleanup));
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
            // Play Button
            parent
                .spawn_bundle(button_bundle(MainMenuSceneButton::Play, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuSceneButton::Play, &asset_server));
                })
                .insert(MainMenuSceneButton::Play);

            // Demos Button
            parent
                .spawn_bundle(button_bundle(MainMenuSceneButton::Demos, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuSceneButton::Demos, &asset_server));
                })
                .insert(MainMenuSceneButton::Demos);

            // Setting Button
            parent
                .spawn_bundle(button_bundle(MainMenuSceneButton::Setting, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuSceneButton::Setting, &asset_server));
                })
                .insert(MainMenuSceneButton::Setting);

            // Quit Button
            parent
                .spawn_bundle(button_bundle(MainMenuSceneButton::Quit, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuSceneButton::Quit, &asset_server));
                })
                .insert(MainMenuSceneButton::Quit);
        })
        .id();

    commands.insert_resource(MainMenuSceneData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands.entity(main_menu_scene_data.ui_root).despawn_recursive();
    commands.entity(main_menu_scene_data.camera_entity).despawn_recursive();
}

fn root(asset_server: &Res<AssetServer>) -> NodeBundle {
    NodeBundle {
        style: Style {
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
        MainMenuSceneButton::Demos => BUTTON_POSITIONS[1],
        MainMenuSceneButton::Setting => BUTTON_POSITIONS[2],
        MainMenuSceneButton::Quit => BUTTON_POSITIONS[3],
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
        image: UiImage(asset_server.load(NORMAL_BUTTON_IMAGE)),
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
                *color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::GREEN.into();
                *color = Color::GREEN.into();
            }
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED.into();
                *color = Color::RED.into();
                match button {
                    MainMenuSceneButton::Setting => state
                        .set(ApplicationScene::SettingScene)
                        .expect("Couldn't switch state to Setting Menu"),
                    MainMenuSceneButton::Play => state
                        .set(ApplicationScene::LoadingScene)
                        .expect("Couldn't switch state to Loading Screen"),
                    _ => exit.send(AppExit),
                }
            }
        }
    }
}

fn text_bundle(main_menu_scene_button: MainMenuSceneButton, asset_server: &Res<AssetServer>) -> TextBundle {
    let text: &str = match main_menu_scene_button {
        MainMenuSceneButton::Play => "PLAY",
        MainMenuSceneButton::Demos => "DEMOS",
        MainMenuSceneButton::Setting => "SETTING",
        MainMenuSceneButton::Quit => "QUIT",
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(HAEDUS_FONT),
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

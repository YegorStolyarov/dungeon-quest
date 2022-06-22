use bevy::prelude::*;

use crate::config::*;
use crate::resources::setting::Setting;
use crate::resources::scene::ApplicationScene;

const RETURN_HOME_BUTTON_SIDE: f32 = 30.0;
const BUTTON_SIDE: f32 = 50.0;

const TEXT_FONT_SIZE: f32 = 50.0;

const BUTTON_POSITIONS: [[f32; 2]; 4] = [
    [20.0, 10.0],   // ReturnHome
    [500.0, 50.0],  // Enable Sound
    [500.0, 110.0], // Enable Music
    [500.0, 170.0], // FullScreen
];

const TEXT_POSITIONS: [[f32; 2]; 3] = [
    [100.0, 50.0],  // Enable Sound
    [100.0, 110.0], // Enable Music
    [100.0, 170.0], // FullScreen
];

#[derive(Component, PartialEq)]
enum SettingSceneButton {
    ReturnHome,
    EnableSound,
    EnableMusic,
    FullScreen,
}

pub struct SettingScenePlugin;

struct SettingSceneData {
    camera_entity: Entity,
    ui_root: Entity,
}

impl Plugin for SettingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationScene::SettingScene).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationScene::SettingScene).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(ApplicationScene::SettingScene).with_system(button_handle_system),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, setting: Res<Setting>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&asset_server))
        .with_children(|parent| {
            parent
                .spawn_bundle(button_bundle(
                    SettingSceneButton::ReturnHome,
                    &asset_server,
                    &setting,
                ))
                .insert(SettingSceneButton::ReturnHome);

            parent.spawn_bundle(text_bundle(SettingSceneButton::EnableSound, &asset_server));
            parent
                .spawn_bundle(check_box_bundle(
                    SettingSceneButton::EnableSound,
                    &asset_server,
                ))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_bundle(
                            SettingSceneButton::EnableSound,
                            &asset_server,
                            &setting,
                        ))
                        .insert(SettingSceneButton::EnableSound);
                });

            parent.spawn_bundle(text_bundle(SettingSceneButton::EnableMusic, &asset_server));
            parent
                .spawn_bundle(check_box_bundle(
                    SettingSceneButton::EnableMusic,
                    &asset_server,
                ))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_bundle(
                            SettingSceneButton::EnableMusic,
                            &asset_server,
                            &setting,
                        ))
                        .insert(SettingSceneButton::EnableMusic);
                });

            parent.spawn_bundle(text_bundle(SettingSceneButton::FullScreen, &asset_server));
            parent
                .spawn_bundle(check_box_bundle(
                    SettingSceneButton::FullScreen,
                    &asset_server,
                ))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(button_bundle(
                            SettingSceneButton::FullScreen,
                            &asset_server,
                            &setting,
                        ))
                        .insert(SettingSceneButton::FullScreen);
                });
        })
        .id();

    commands.insert_resource(SettingSceneData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, setting_scene_data: Res<SettingSceneData>, setting: Res<Setting>) {
    setting.store();
    commands.entity(setting_scene_data.ui_root).despawn_recursive();
    commands.entity(setting_scene_data.camera_entity).despawn_recursive();
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

fn check_box_bundle(
    setting_scene_button: SettingSceneButton,
    asset_server: &Res<AssetServer>,
) -> NodeBundle {
    let size = match setting_scene_button {
        SettingSceneButton::ReturnHome => Size::new(
            Val::Px(RETURN_HOME_BUTTON_SIDE),
            Val::Px(RETURN_HOME_BUTTON_SIDE),
        ),
        _ => Size::new(Val::Px(BUTTON_SIDE), Val::Px(BUTTON_SIDE)),
    };

    let position: [f32; 2] = match setting_scene_button {
        SettingSceneButton::ReturnHome => BUTTON_POSITIONS[0],
        SettingSceneButton::EnableSound => BUTTON_POSITIONS[1],
        SettingSceneButton::EnableMusic => BUTTON_POSITIONS[2],
        SettingSceneButton::FullScreen => BUTTON_POSITIONS[3],
    };

    NodeBundle {
        style: Style {
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(position[0]),
                top: Val::Px(position[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load(NORMAL_BUTTON_IMAGE)),
        ..Default::default()
    }
}

fn button_bundle(
    setting_scene_button: SettingSceneButton,
    asset_server: &Res<AssetServer>,
    setting: &Res<Setting>,
) -> ButtonBundle {
    let size = Size::new(Val::Px(30.0), Val::Px(30.0));

    let image_str: &str = match setting_scene_button {
        SettingSceneButton::ReturnHome => HOME_ICON,
        _ => TICK_ICON,
    };

    let is_visible: bool = match setting_scene_button {
        SettingSceneButton::EnableSound => setting.get_enable_sound(),
        SettingSceneButton::EnableMusic => setting.get_enable_music(),
        SettingSceneButton::FullScreen => setting.get_fullscreen(),
        _ => true,
    };

    ButtonBundle {
        visibility: Visibility { is_visible },
        style: Style {
            size,
            ..Default::default()
        },
        image: UiImage(asset_server.load(image_str)),
        ..Default::default()
    }
}

fn button_handle_system(
    mut button_query: Query<
        (
            &Interaction,
            &SettingSceneButton,
            &mut UiColor,
            &mut Visibility,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<ApplicationScene>>,
    mut setting: ResMut<Setting>,
) {
    for (interaction, button, mut color, mut visibility) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::Clicked => match button {
                SettingSceneButton::ReturnHome => state
                    .set(ApplicationScene::MainMenuScene)
                    .expect("Couldn't switch state to Main Menu"),
                SettingSceneButton::EnableSound => {
                    visibility.is_visible = !visibility.is_visible;
                    setting.set_enable_sound(visibility.is_visible);
                }
                SettingSceneButton::EnableMusic => {
                    visibility.is_visible = !visibility.is_visible;
                    setting.set_enable_music(visibility.is_visible);
                }
                SettingSceneButton::FullScreen => {
                    visibility.is_visible = !visibility.is_visible;
                    setting.set_fullscreen(visibility.is_visible);
                }
            },
        }
    }
}

fn text_bundle(
    setting_scene_button: SettingSceneButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match setting_scene_button {
        SettingSceneButton::EnableSound => "Enable Sound",
        SettingSceneButton::EnableMusic => "Enable Music",
        SettingSceneButton::FullScreen => "Fullscreen",
        SettingSceneButton::ReturnHome => "_",
    };

    let position = match setting_scene_button {
        SettingSceneButton::EnableSound => TEXT_POSITIONS[0],
        SettingSceneButton::EnableMusic => TEXT_POSITIONS[1],
        SettingSceneButton::FullScreen => TEXT_POSITIONS[2],
        _ => TEXT_POSITIONS[0],
    };

    TextBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(position[0]),
                top: Val::Px(position[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(HAEDUS_FONT),
                font_size: TEXT_FONT_SIZE,
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

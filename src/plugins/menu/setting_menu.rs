use bevy::prelude::*;

use crate::config::*;
use crate::state::*;

const BUTTON_SIDE: f32 = 30.0;

const BIG_FONT_SIZE: f32 = 50.0;
const SMALL_FONT_SIZE: f32 = 30.0;

const BUTTON_POSITIONS: [[f32; 2]; 1] = [
    [20.0, 10.0], // ReturnHome
];

#[derive(Component, PartialEq)]
enum SettingMenuButton {
    ReturnHome,
    EnableSound,
    EnableMusic,
    FullScreen,
}

pub struct SettingMenuPlugin;

struct SettingMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

impl Plugin for SettingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::SettingMenu).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationState::SettingMenu).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&asset_server))
        .with_children(|parent| {
            parent
                .spawn_bundle(button_bundle(SettingMenuButton::ReturnHome, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(image_bundle(SettingMenuButton::ReturnHome, &asset_server));
                });
        })
        .id();

    commands.insert_resource(SettingMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<SettingMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
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
    setting_menu_button: SettingMenuButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = match setting_menu_button {
        _ => Size::new(Val::Px(BUTTON_SIDE), Val::Px(BUTTON_SIDE)),
    };

    let possition: [f32; 2] = match setting_menu_button {
        SettingMenuButton::ReturnHome => BUTTON_POSITIONS[0],
        _ => BUTTON_POSITIONS[0],
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
        image: UiImage(asset_server.load(SMALL_BUTTON_IMAGE)),
        ..Default::default()
    }
}

fn image_bundle(
    setting_menu_button: SettingMenuButton,
    asset_server: &Res<AssetServer>,
) -> ImageBundle {
    let size = match setting_menu_button {
        _ => Size::new(Val::Px(24.0), Val::Px(24.0)),
    };

    let image_str: &str = match setting_menu_button {
        SettingMenuButton::ReturnHome => HOME_ICON,
        _ => "",
    };

    ImageBundle {
        style: Style {
            size,
            ..Default::default()
        },
        image: UiImage(asset_server.load(image_str)),
        ..Default::default()
    }
}

fn text_bundle(
    setting_menu_button: SettingMenuButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match setting_menu_button {
        SettingMenuButton::EnableSound => "Enable Sound",
        SettingMenuButton::EnableMusic => "Enable Music",
        SettingMenuButton::FullScreen => "Enable Fullscreen",
        SettingMenuButton::ReturnHome => "[Return]",
        _ => "",
    };

    let font_size: f32 = match setting_menu_button {
        _ => BIG_FONT_SIZE,
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load(HAEDUS_FONT),
                font_size,
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

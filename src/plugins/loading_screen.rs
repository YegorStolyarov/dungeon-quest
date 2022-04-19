use bevy::prelude::*;

use crate::config::*;
use crate::state::*;

const LOADING_BORDER_WIDTH: f32 = 400.0;
const LOADING_BORDER_HEIGHT: f32 = 50.0;

const TEXT_FONT_SIZE: f32 = 50.0;
const TEXT_POSITION: [f32; 2] = [270.0, WINDOW_HEIGHT / 2.0 - LOADING_BORDER_HEIGHT / 2.0];

#[derive(Component)]
struct Loader {
    max_width: f32,
    current_width: f32,
}

struct LoadingScreenData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::LoadingScreen).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(ApplicationState::LoadingScreen).with_system(run_loading),
        );
        app.add_system_set(
            SystemSet::on_exit(ApplicationState::LoadingScreen).with_system(cleanup),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
    let ui_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            // Border
            parent
                .spawn_bundle(border_bundle())
                .with_children(|parent| {
                    // Loader
                    parent.spawn_bundle(loader_bundle()).insert(Loader {
                        max_width: LOADING_BORDER_WIDTH - 10.0,
                        current_width: 0.0,
                    });
                });

            // Loading Text
            parent.spawn_bundle(loading_text_bundle(asset_server));
        })
        .id();

    commands.insert_resource(LoadingScreenData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<LoadingScreenData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: UiColor(Color::BLACK),
        ..Default::default()
    }
}

fn border_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(LOADING_BORDER_WIDTH),
                Val::Px(LOADING_BORDER_HEIGHT),
            ),
            position: Rect {
                top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0)),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        color: UiColor(Color::WHITE),
        ..Default::default()
    }
}

fn loader_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(0.0),
                Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
            ),
            position: Rect {
                left: Val::Px(5.0),
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
            },
            ..Default::default()
        },
        color: UiColor(Color::RED),
        ..Default::default()
    }
}

fn loading_text_bundle(asset_server: Res<AssetServer>) -> TextBundle {
    TextBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(TEXT_POSITION[0]),
                top: Val::Px(TEXT_POSITION[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Loading".to_string(),
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

fn run_loading(mut query: Query<(&mut Loader, &mut Style)>) {
    for (mut loader, mut style) in query.iter_mut() {
        if loader.current_width < loader.max_width {
            loader.current_width += 2.5;
            style.size.width = Val::Px(loader.current_width);
        }
    }
}

use bevy::prelude::*;

mod ui;

use crate::state::*;
use ui::*;

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::MainMenu).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationState::MainMenu).with_system(cleanup));
        // app.add_system_to_stage(
        // ApplicationState::MainMenu,
        // menu_button_interaction_handle_system,
        // );
        app.add_system_set(
            SystemSet::on_update(ApplicationState::MainMenu)
                .with_system(button_interaction_handle_system)
                .with_system(button_on_click_handle_system),
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
                .spawn_bundle(button_bundle(MainMenuButton::Play, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuButton::Play, &asset_server));
                })
                .insert(MainMenuButton::Play);

            // Demos Button
            parent
                .spawn_bundle(button_bundle(MainMenuButton::Demos, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuButton::Demos, &asset_server));
                })
                .insert(MainMenuButton::Demos);

            // Setting Button
            parent
                .spawn_bundle(button_bundle(MainMenuButton::Setting, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuButton::Setting, &asset_server));
                })
                .insert(MainMenuButton::Setting);

            // Quit Button
            parent
                .spawn_bundle(button_bundle(MainMenuButton::Quit, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(MainMenuButton::Quit, &asset_server));
                })
                .insert(MainMenuButton::Quit);
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

use bevy::prelude::*;

pub mod ui;

use crate::state::*;
use ui::*;

pub struct DemosMenuPlugin;

impl Plugin for DemosMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ApplicationState::DemosMenu).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(ApplicationState::DemosMenu).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(ApplicationState::DemosMenu).with_system(cleanup));
    }
}

struct DemosMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
    let ui_root = commands
        .spawn_bundle(root(&asset_server))
        .with_children(|parent| {
            parent
                .spawn_bundle(button_bundle(DemosMenuButton::Movement, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(DemosMenuButton::Movement, &asset_server));
                })
                .insert(DemosMenuButton::Movement);

            parent
                .spawn_bundle(button_bundle(DemosMenuButton::Home, &asset_server))
                .with_children(|parent| {
                    parent.spawn_bundle(text_bundle(DemosMenuButton::Home, &asset_server));
                })
                .insert(DemosMenuButton::Home);
        })
        .id();

    commands.insert_resource(DemosMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<DemosMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}

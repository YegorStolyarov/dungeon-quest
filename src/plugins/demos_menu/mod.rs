use bevy::prelude::*;

use crate::state::*;

pub struct DemosMenuPlugin;

impl Plugin for DemosMenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(button_system);
        // app.add_system(button_press_system);
        app.add_system_set(SystemSet::on_enter(ApplicationState::DemosMenu).with_system(setup));
        app.add_system_set(SystemSet::on_exit(ApplicationState::DemosMenu).with_system(cleanup));
    }
}

struct DemosMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
    let ui_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(Color::RED),
            // image: UiImage(asset_server.load(BACKGROUND_IMAGE)),
            ..Default::default()
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

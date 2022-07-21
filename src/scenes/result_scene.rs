use bevy::prelude::*;

use crate::config::*;
use crate::resources::dictionary::Dictionary;
use crate::resources::materials::scenes::MenuBoxMaterials;
use crate::resources::materials::scenes::ScenesMaterials;
use crate::resources::materials::Materials;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

use crate::ingame::resources::profile::Profile;

const RETURN_BUTTON_SIDE: f32 = 50.0;

const MENU_BOX_TILE_SIZE: f32 = 60.0;
const MENU_BOX_WIDTH_TILES: f32 = 8.0;
const MENU_BOX_HEIGHT_TILES: f32 = 6.0;

const MENU_BOX_ARRAY: [[i8; 8]; 6] = [
    [0, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 8],
];

struct ResultSceneData {
    user_interface_root: Entity,
}

pub struct ResultScenePlugin;

impl Plugin for ResultScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::ResultScene).with_system(setup));
        // app.add_system_set(SystemSet::on_update(SceneState::ResultScene));
        app.add_system_set(SystemSet::on_exit(SceneState::ResultScene).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    scenes_materials: Res<ScenesMaterials>,
    profile: Res<Profile>,
    dictionary: Res<Dictionary>,
) {
    // user interface root
    let user_interface_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            options_menu_box(parent, &scenes_materials.menu_box_materials);
        })
        .id();
    commands.insert_resource(ResultSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, result_scene_data: Res<ResultSceneData>, setting: Res<Setting>) {
    setting.store();
    commands
        .entity(result_scene_data.user_interface_root)
        .despawn_recursive();
}

fn root(materials: &Materials) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(materials.sub_menu_background.clone()),
        ..Default::default()
    }
}

fn options_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(MENU_BOX_WIDTH_TILES),
        height: Val::Px(MENU_BOX_WIDTH_TILES),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - MENU_BOX_TILE_SIZE * MENU_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - MENU_BOX_TILE_SIZE * MENU_BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in MENU_BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: Rect<Val> = Rect {
                left: Val::Px(start_left + MENU_BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(start_top + MENU_BOX_TILE_SIZE * row_index as f32),
                bottom: Val::Auto,
                right: Val::Auto,
            };

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

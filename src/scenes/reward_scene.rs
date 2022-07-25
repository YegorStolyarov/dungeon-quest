use bevy::prelude::*;
use std::time::Duration;

use crate::config::*;
use crate::materials::scenes::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::materials::Materials;
use crate::resources::dictionary::Dictionary;
use crate::scenes::SceneState;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 4.0;
const BOX_HEIGHT_TILES: f32 = 3.0;

const BOX_ARRAY: [[i8; 5]; 3] = [[0, 1, 1, 1, 2], [3, 4, 4, 4, 5], [6, 7, 7, 7, 8]];

struct RewardSceneData {
    user_interface_root: Entity,
}

#[derive(Component)]
struct RewardSceneCountDown(Timer);

pub struct RewardScenePlugin;

impl Plugin for RewardScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::RewardScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::RewardScene).with_system(colddown_handle),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::RewardScene).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    scenes_materials: Res<ScenesMaterials>,
    dictionary: Res<Dictionary>,
) {
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            button(parent, &materials, &dictionary);
        })
        .insert(Name::new("RewardSceneUI"))
        .id();

    commands.insert_resource(RewardSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, reward_scene_data: Res<RewardSceneData>) {
    commands
        .entity(reward_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(BOX_TILE_SIZE),
        height: Val::Px(BOX_TILE_SIZE),
    };

    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    root.spawn_bundle(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        for (row_index, row) in BOX_ARRAY.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                let position: Rect<Val> = Rect {
                    left: Val::Px(start_left + BOX_TILE_SIZE * column_index as f32),
                    top: Val::Px(start_top + BOX_TILE_SIZE * row_index as f32),
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

                parent.spawn_bundle(NodeBundle {
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
    })
    .insert(Name::new("MenuBox"));
}

fn button(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    // let glossary = dictionary.get_glossary();

    root.spawn_bundle(ButtonBundle {
        style: Style {
            position: Rect {
                left: Val::Px(435.0),
                top: Val::Px(300.0),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            size: Size {
                width: Val::Px(150.0),
                height: Val::Px(35.0),
            },
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 35.0,
                    color: Color::DARK_GRAY,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    })
    .insert(Name::new("Reward"))
    .insert(RewardSceneCountDown(Timer::new(
        Duration::from_secs(2),
        false,
    )));
}

fn colddown_handle(
    time: Res<Time>,
    mut countdown_query: Query<&mut RewardSceneCountDown>,
    mut state: ResMut<State<SceneState>>,
) {
    let mut countdown = countdown_query.single_mut();
    countdown.0.tick(time.delta());
    if countdown.0.finished() {
        state.pop().unwrap();
    }
}

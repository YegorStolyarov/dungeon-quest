// use bevy::prelude::*;
// use std::slice::Iter;
//
// use crate::config::*;
// use crate::resources::dictionary::Dictionary;
// use crate::resources::materials::scenes::MenuBoxMaterials;
// use crate::resources::materials::scenes::ScenesMaterials;
// use crate::resources::materials::Materials;
// use crate::resources::setting::Setting;
// use crate::scenes::SceneState;
//
// const RETURN_HOME_BUTTON_SIDE: f32 = 50.0;
// const BUTTON_SIDE: f32 = 50.0;
// const OPTIONS_MENU_BOX_TILE_SIZE: f32 = 60.0;
//
// const OPTIONS_MENU_BOX_WIDTH_TILES: f32 = 8.0;
// const OPTIONS_MENU_BOX_HEIGHT_TILES: f32 = 6.0;
//
// const OPTIONS_MENU_BOX_ARRAY: [[i8; 8]; 6] = [
//     [0, 1, 1, 1, 1, 1, 1, 2],
//     [3, 4, 4, 4, 4, 4, 4, 5],
//     [3, 4, 4, 4, 4, 4, 4, 5],
//     [3, 4, 4, 4, 4, 4, 4, 5],
//     [3, 4, 4, 4, 4, 4, 4, 5],
//     [6, 7, 7, 7, 7, 7, 7, 8],
// ];
//
// // const BUTTON_POSITIONS: [[f32; 2]; 4] = [
// //     [20.0, 10.0],   // ReturnHome
// //     [500.0, 50.0],  // Enable Sound
// //     [500.0, 110.0], // Enable Music
// //     [500.0, 170.0], // FullScreen
// // ];
//
// #[derive(Component, PartialEq)]
// enum OptionsSceneButton {
//     Return,
//     EnableSound,
//     EnableMusic,
//     Vietnamese,
//     English,
// }
//
// impl OptionsSceneButton {
//     pub fn iterator() -> Iter<'static, OptionsSceneButton> {
//         static OPTIONS_SCENE_BUTTONS: [OptionsSceneButton; 5] = [
//             OptionsSceneButton::Return,
//             OptionsSceneButton::EnableSound,
//             OptionsSceneButton::EnableMusic,
//             OptionsSceneButton::Vietnamese,
//             OptionsSceneButton::English,
//         ];
//         OPTIONS_SCENE_BUTTONS.iter()
//     }
// }
//
// pub struct OptionsScenePlugin;
//
// struct OptionsSceneData {
//     user_interface_root: Entity,
// }
//
// impl Plugin for OptionsScenePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(SystemSet::on_enter(SceneState::OptionsScene).with_system(setup));
//         // app.add_system_set(
//         // SystemSet::on_update(SceneState::OptionsScene).with_system(button_handle_system),
//         // );
//         app.add_system_set(SystemSet::on_exit(SceneState::OptionsScene).with_system(cleanup));
//     }
// }
//
// fn setup(
//     mut commands: Commands,
//     materials: Res<Materials>,
//     scenes_materials: Res<ScenesMaterials>,
//     setting: Res<Setting>,
//     dictionary: Res<Dictionary>,
// ) {
//     // user interface root
//     let user_interface_root = commands
//         .spawn_bundle(root(&materials))
//         .with_children(|parent| {
//             options_menu_box(parent, &scenes_materials.menu_box_materials);
//             texts(parent, &materials, &dictionary);
//             buttons(parent, &setting, &scenes_materials, &dictionary);
//         })
//         .id();
//     commands.insert_resource(OptionsSceneData {
//         user_interface_root,
//     });
// }
//
// fn cleanup(
//     mut commands: Commands,
//     setting_scene_data: Res<OptionsSceneData>,
//     setting: Res<Setting>,
// ) {
//     setting.store();
//     commands
//         .entity(setting_scene_data.user_interface_root)
//         .despawn_recursive();
// }
//
// fn root(materials: &Materials) -> NodeBundle {
//     NodeBundle {
//         style: Style {
//             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//             ..Default::default()
//         },
//         image: UiImage(materials.sub_menu_background.clone()),
//         ..Default::default()
//     }
// }
//
// fn options_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
//     let size: Size<Val> = Size {
//         width: Val::Px(OPTIONS_MENU_BOX_TILE_SIZE),
//         height: Val::Px(OPTIONS_MENU_BOX_TILE_SIZE),
//     };
//
//     let start_left = (WINDOW_HEIGHT * RESOLUTION
//         - OPTIONS_MENU_BOX_TILE_SIZE * OPTIONS_MENU_BOX_WIDTH_TILES)
//         / 2.0;
//
//     let start_top =
//         (WINDOW_HEIGHT - OPTIONS_MENU_BOX_TILE_SIZE * OPTIONS_MENU_BOX_HEIGHT_TILES) / 2.0;
//
//     for (row_index, row) in OPTIONS_MENU_BOX_ARRAY.iter().enumerate() {
//         for (column_index, value) in row.iter().enumerate() {
//             let position: Rect<Val> = Rect {
//                 left: Val::Px(start_left + OPTIONS_MENU_BOX_TILE_SIZE * column_index as f32),
//                 top: Val::Px(start_top + OPTIONS_MENU_BOX_TILE_SIZE * row_index as f32),
//                 bottom: Val::Auto,
//                 right: Val::Auto,
//             };
//
//             let image: Handle<Image> = match value {
//                 0 => menu_box_materials.top_right.clone(),
//                 1 => menu_box_materials.top_center.clone(),
//                 2 => menu_box_materials.top_left.clone(),
//                 3 => menu_box_materials.mid_right.clone(),
//                 4 => menu_box_materials.mid_center.clone(),
//                 5 => menu_box_materials.mid_left.clone(),
//                 6 => menu_box_materials.bottom_right.clone(),
//                 7 => menu_box_materials.bottom_center.clone(),
//                 8 => menu_box_materials.bottom_left.clone(),
//                 _ => panic!("Unknown resources"),
//             };
//
//             root.spawn_bundle(NodeBundle {
//                 image: UiImage(image),
//                 style: Style {
//                     position_type: PositionType::Absolute,
//                     position,
//                     size,
//                     ..Default::default()
//                 },
//
//                 ..Default::default()
//             });
//         }
//     }
// }
//
// fn texts(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
//     let font = materials.get_font(dictionary.get_current_language());
//     let glossary = dictionary.get_glossary();
//
//     let position_of_texts: [[f32; 2]; 4] = [
//         [440.0, 160.0],
//         [320.0, 230.0],
//         [320.0, 290.0],
//         [320.0, 350.0],
//     ];
//
//     for index in 0..position_of_texts.len() {
//         let value: String = match index {
//             0 => glossary.options_scene_text.options.clone(),
//             1 => glossary.options_scene_text.enable_music.clone(),
//             2 => glossary.options_scene_text.enable_sound.clone(),
//             3 => glossary.options_scene_text.language.clone(),
//             _ => panic!("Unknown text"),
//         };
//
//         let font_size: f32 = match index {
//             0 => 50.0,
//             _ => 30.0,
//         };
//
//         root.spawn_bundle(TextBundle {
//             style: Style {
//                 position_type: PositionType::Absolute,
//                 position: Rect {
//                     left: Val::Px(position_of_texts[index][0]),
//                     top: Val::Px(position_of_texts[index][1]),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             text: Text::with_section(
//                 value,
//                 TextStyle {
//                     font: font.clone(),
//                     font_size,
//                     color: Color::BLACK,
//                 },
//                 TextAlignment {
//                     vertical: VerticalAlign::Center,
//                     horizontal: HorizontalAlign::Center,
//                 },
//             ),
//             ..Default::default()
//         });
//     }
// }
//
// // fn buttons(
// //     root: &mut ChildBuilder,
// //     setting: &Setting,
// //     scenes_materials: &ScenesMaterials,
// //     dictionary: &Dictionary,
// // ) {
// //     for (index, button) in OptionsSceneButton::iterator().enumerate() {
// //
// //     }
// // }

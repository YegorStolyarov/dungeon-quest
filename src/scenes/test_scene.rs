// use bevy::prelude::*;

// use crate::scenes::{ApplicationScene, ApplicationSceneController};

// pub struct TestScenePlugin;

// impl Plugin for TestScenePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(SystemSet::on_enter(ApplicationScene::TestScene).with_system(setup));
//     }
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let images: [UiImage; 9] = [
//         UiImage(asset_server.load("images/walls/wall_top_mid.png")),
//         UiImage(asset_server.load("images/walls/wall_mid.png")),
//         UiImage(asset_server.load("images/floors/floor_1.png")),
//         UiImage(asset_server.load("images/walls/wall_side_mid_right.png")),
//         UiImage(asset_server.load("images/walls/wall_side_mid_left.png")),
//         UiImage(asset_server.load("images/walls/wall_side_top_left.png")),
//         UiImage(asset_server.load("images/walls/wall_side_top_right.png")),
//         UiImage(asset_server.load("images/walls/wall_corner_bottom_right.png")),
//         UiImage(asset_server.load("images/walls/wall_corner_bottom_left.png")),
//     ];

//     commands.spawn_bundle(UiCameraBundle::default());
//     commands
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                 ..Default::default()
//             },
//             color: UiColor(Color::BLACK),
//             ..Default::default()
//         })
//         .with_children(|parent| {

//         });
// }

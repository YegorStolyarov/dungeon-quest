// use bevy::prelude::*;
// use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
//
// use crate::components::player::PlayerComponent;
// use crate::resources::dungeon::block_type::BlockType;
// use crate::resources::dungeon::door::Door;
//
// pub struct DebugPlugin;
//
// impl Plugin for DebugPlugin {
//     fn build(&self, app: &mut App) {
//         if cfg!(debug_assertions) {
//             app.add_plugin(WorldInspectorPlugin::new())
//                 .register_inspectable::<Door>()
//                 .register_inspectable::<BlockType>()
//                 .register_inspectable::<PlayerComponent>();
//         }
//     }
// }

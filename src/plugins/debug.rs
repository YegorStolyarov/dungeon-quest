use bevy::prelude::*;
// use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::ingame::resources::dungeon::door::Door;
// use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Door>();
            // app.add_plugin(InspectorPlugin::<PlayerDungeonStats>::new());
        }
    }
}

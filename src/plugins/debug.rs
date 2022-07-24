use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::ingame::resources::dungeon::block_type::BlockType;
use crate::ingame::resources::dungeon::door::Door;
use crate::ingame::resources::player::Player;
// use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Door>()
                .register_inspectable::<BlockType>()
                .register_inspectable::<Player>();
        }
    }
}

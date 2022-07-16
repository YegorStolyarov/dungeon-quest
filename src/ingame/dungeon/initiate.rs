use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;

pub fn initiate_dungeon(mut commands: Commands) {
    commands.insert_resource(Dungeon::new());
}

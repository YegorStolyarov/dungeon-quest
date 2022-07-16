use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::fixed::data::Data;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::profile::Profile;

pub fn initiate_dungeon(mut commands: Commands) {
    commands.insert_resource(Dungeon::new());
}

pub fn initiate_player(mut commands: Commands, data: Res<Data>, profile: Res<Profile>) {
    let hero_class = profile.hero_class.clone();
    let player = Player::new(hero_class, data.clone());
    commands.insert_resource(player);
}

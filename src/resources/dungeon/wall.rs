use bevy::prelude::*;

use crate::resources::dungeon::wall_type::WallType;

#[derive(Component)]
pub struct Wall {
    pub wall_type: WallType,
    pub row_index: usize,
    pub column_index: usize,
    pub value: i32,
}

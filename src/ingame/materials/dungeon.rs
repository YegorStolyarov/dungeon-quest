use bevy::prelude::*;

#[derive(Clone)]
pub struct DungeonMaterials {
    pub floor: Handle<Image>,
    pub wall: Handle<Image>,
    pub wall_border_mid: Handle<Image>,
    pub wall_border_corner_bottom_left: Handle<Image>,
    pub wall_border_corner_bottom_right: Handle<Image>,
    pub wall_border_left: Handle<Image>,
    pub wall_border_right: Handle<Image>,
    pub wall_border_corner_top_left: Handle<Image>,
    pub wall_border_corner_top_right: Handle<Image>,
    pub wall_border_corner_left: Handle<Image>,
    pub wall_border_corner_right: Handle<Image>,
}

use bevy::prelude::*;

#[derive(Clone)]
pub struct DungeonMaterials {
    pub floor: Handle<Image>,
    pub ladder: Handle<Image>,
    pub wall: Handle<Image>,
    pub treasure: Handle<Image>,
    pub wall_border_mid: Handle<Image>,
    pub wall_border_corner_bottom_left: Handle<Image>,
    pub wall_border_corner_bottom_right: Handle<Image>,
    pub wall_border_left: Handle<Image>,
    pub wall_border_right: Handle<Image>,
    pub wall_border_corner_top_left: Handle<Image>,
    pub wall_border_corner_top_right: Handle<Image>,
    pub wall_border_corner_left: Handle<Image>,
    pub wall_border_corner_right: Handle<Image>,
    pub wall_left: Handle<Image>,
    pub wall_right: Handle<Image>,
    pub door_left_part: Handle<Image>,
    pub door_right_part: Handle<Image>,
    pub door_top_part: Handle<Image>,
    pub door_opened: Handle<Image>,
    pub door_closed: Handle<Image>,
}

use bevy::prelude::*;

#[derive(Clone)]
pub struct MenuBoxMaterials {
    pub top_right: Handle<Image>,
    pub top_center: Handle<Image>,
    pub top_left: Handle<Image>,
    pub mid_right: Handle<Image>,
    pub mid_center: Handle<Image>,
    pub mid_left: Handle<Image>,
    pub bottom_right: Handle<Image>,
    pub bottom_center: Handle<Image>,
    pub bottom_left: Handle<Image>,
}

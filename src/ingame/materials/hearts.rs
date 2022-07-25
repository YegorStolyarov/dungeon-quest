use bevy::prelude::*;

#[derive(Clone)]
pub struct HeartsMaterials {
    pub full_heart: Handle<Image>,
    pub half_heart: Handle<Image>,
    pub empty_heart: Handle<Image>,
}

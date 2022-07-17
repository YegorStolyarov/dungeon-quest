use bevy::prelude::*;

#[derive(Component)]
pub enum BorderLayer {
    Left,
    Right,
    Bottom,
    Top,
}

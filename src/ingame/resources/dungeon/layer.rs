use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub enum Layer {
    None,
    BorderLeft,
    BorderRight,
    BorderBottom,
    BorderTop,
}

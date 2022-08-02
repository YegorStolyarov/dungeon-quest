use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::slice::Iter;

#[derive(Component, PartialEq, Eq, Clone, Inspectable, Debug)]
pub enum Door {
    Left,
    Right,
    Top,
    Bottom,
}

impl Door {
    pub fn iterator() -> Iter<'static, Door> {
        [Door::Left, Door::Right, Door::Top, Door::Bottom].iter()
    }
}

#[derive(PartialEq, Eq, Component, Clone, Debug)]
pub enum VerticaltDoor {
    Top,
    Bottom,
}

#[derive(PartialEq, Eq, Component, Clone)]
pub struct HorizontalDoor;

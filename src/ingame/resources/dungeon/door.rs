use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::slice::Iter;

#[derive(PartialEq, Eq, Component, Clone, Inspectable)]
pub enum Door {
    Left,
    Right,
    Top,
    Bottom,
}

impl Door {
    pub fn iterator() -> Iter<'static, Door> {
        static DOORS: [Door; 4] = [Door::Left, Door::Right, Door::Top, Door::Bottom];
        DOORS.iter()
    }
}

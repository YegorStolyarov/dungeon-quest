use bevy::prelude::*;
use std::slice::Iter;

#[derive(PartialEq, Eq, Component, Clone)]
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

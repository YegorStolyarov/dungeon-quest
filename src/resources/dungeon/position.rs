use bevy_inspector_egui::Inspectable;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Inspectable, Default)]
pub struct Position {
    pub row_index: usize,
    pub column_index: usize,
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row_index.hash(state);
        self.column_index.hash(state);
    }
}

use bevy::prelude::*;

use crate::state::ApplicationState;

pub struct ApplicationStatus {
    is_data_loaded: bool,
    next_state: ApplicationState,
    previous_state: ApplicationState,
}

impl ApplicationStatus {
    pub fn new() -> Self {
        ApplicationStatus {
            is_data_loaded: false,
            next_state: ApplicationState::MainMenu,
            previous_state: ApplicationState::MainMenu,
        }
    }

    pub fn data_loaded(&mut self) {
        self.is_data_loaded = true;
    }

    pub fn is_data_loaded(&self) -> bool {
        self.is_data_loaded
    }

    pub fn set_next_state(&mut self, next_state: ApplicationState) {
        self.next_state = next_state;
    }

    pub fn _set_previous_state(&mut self, previous_state: ApplicationState) {
        self.previous_state = previous_state;
    }

    pub fn get_next_state(&mut self) -> ApplicationState {
        let next_state: ApplicationState = self.next_state.clone();
        self.next_state = ApplicationState::MainMenu;
        next_state
    }

    pub fn _get_previous_state(&mut self) -> ApplicationState {
        let previous_state: ApplicationState = self.previous_state.clone();
        self.previous_state = ApplicationState::MainMenu;
        previous_state
    }
}

impl FromWorld for ApplicationStatus {
    fn from_world(_world: &mut World) -> Self {
        ApplicationStatus::new()
    }
}

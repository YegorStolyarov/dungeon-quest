use bevy::prelude::*;

// pub mod loading_scene;
pub mod main_menu_scene;
// pub mod setting_scene;
// pub mod test_scene;

pub struct ApplicationSceneController {
    next_state: ApplicationScene,
    previous_state: ApplicationScene,
}

impl ApplicationSceneController {
    pub fn new() -> Self {
        ApplicationSceneController {
            next_state: ApplicationScene::MainMenuScene,
            previous_state: ApplicationScene::MainMenuScene,
        }
    }

    pub fn set_next_state(&mut self, next_state: ApplicationScene) {
        self.next_state = next_state;
    }

    pub fn _set_previous_state(&mut self, previous_state: ApplicationScene) {
        self.previous_state = previous_state;
    }

    pub fn get_next_state(&mut self) -> ApplicationScene {
        let next_state: ApplicationScene = self.next_state.clone();
        self.next_state = ApplicationScene::MainMenuScene;
        next_state
    }

    pub fn _get_previous_state(&mut self) -> ApplicationScene {
        let previous_state: ApplicationScene = self.previous_state.clone();
        self.previous_state = ApplicationScene::MainMenuScene;
        previous_state
    }
}

impl FromWorld for ApplicationSceneController {
    fn from_world(_world: &mut World) -> Self {
        ApplicationSceneController::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ApplicationScene {
    MainMenuScene,
    SettingScene,
    LoadingScene,
    TestScene,
}

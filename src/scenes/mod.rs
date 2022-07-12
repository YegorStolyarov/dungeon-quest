use bevy::prelude::*;

pub mod credits_scene;
pub mod game_mode_select_scene;
pub mod help_scene;
pub mod highscore_scene;
pub mod loading_scene;
pub mod main_menu_scene;
pub mod options_scene;

pub struct SceneController {
    next_scene: SceneState,
    previous_scene: SceneState,
}

impl SceneController {
    pub fn new() -> Self {
        SceneController {
            next_scene: SceneState::MainMenuScene,
            previous_scene: SceneState::MainMenuScene,
        }
    }

    pub fn set_next_state(&mut self, next_scene: SceneState) {
        self.next_scene = next_scene;
    }

    pub fn _set_previous_state(&mut self, previous_scene: SceneState) {
        self.previous_scene = previous_scene;
    }

    pub fn get_next_state(&mut self) -> SceneState {
        let next_scene: SceneState = self.next_scene.clone();
        self.next_scene = SceneState::MainMenuScene;
        next_scene
    }

    pub fn _get_previous_state(&mut self) -> SceneState {
        let previous_scene: SceneState = self.previous_scene.clone();
        self.previous_scene = SceneState::MainMenuScene;
        previous_scene
    }
}

impl FromWorld for SceneController {
    fn from_world(_world: &mut World) -> Self {
        SceneController::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    LoadingScene,
    MainMenuScene,
    HighscoreScene,
    OptionsScene,
    HelpScene,
    CreditsScene,
    GameModeSelectScene,
    HeroSelectScene,
    InGameScene,
    LuckySpinScene,
    RandomRewardScene,
    ResultScene,
    TestScene,
}

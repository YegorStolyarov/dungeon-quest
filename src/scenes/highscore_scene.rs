use bevy::prelude::*;

use crate::scenes::SceneState;

enum HighscoreSceneButton {
    Return,
}

struct HighscoreSceneData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct HighscoreScenePlugin;

impl Plugin for HighscoreScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::MainMenuScene).with_system(setup));
        app.add_system_set(SystemSet::on_exit(SceneState::MainMenuScene).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(SceneState::MainMenuScene).with_system(button_handle_system),
        );
    }
}

fn setup() {}
fn cleanup() {}
fn button_handle_system() {}

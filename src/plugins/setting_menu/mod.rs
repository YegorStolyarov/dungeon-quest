use bevy::prelude::*;

use crate::state::*;

#[derive(Component)]
struct Setting {
    enable_sound: bool,
    enable_music: bool,
    fullscreen: bool,
}
//
// pub struct SettingMenuPlugin;
//
// impl Plugin for SettingMenuPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(setup);
//     }
// }
//
// fn setup(mut command: Commands) {
//     command.insert_resource(Setting {
//         enable_sound: false,
//         enable_music: false,
//         fullscreen: false,
//     })
// }

use bevy::prelude::*;
use bevy_kira_audio::prelude::{AudioSource, Audio, AudioControl};

use crate::resources::setting::Setting;
use crate::scenes::SceneState;

// Our type for the custom audio channel

#[derive(Resource)]
pub struct BackgroundAudioChannel {
    background_music: Handle<AudioSource>,
    loop_started: bool,
    volume: f32,
}

pub fn background_audio_channel_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_audio_channel = BackgroundAudioChannel {
        background_music: asset_server.load("sounds/background_music.ogg"),
        loop_started: false,
        volume: 0.2,
    };

    commands.insert_resource(background_audio_channel);
}

pub fn play_background_music(
    mut background_audio_channel: ResMut<BackgroundAudioChannel>,
    state: Res<State<SceneState>>,
    setting: Res<Setting>,
    audio: Res<Audio>,
) {
    if setting.get_enable_music() {
        if !background_audio_channel.loop_started {
            background_audio_channel.loop_started = true;
            audio.set_volume(
                background_audio_channel.volume as f64,
            );
            audio.play(
                background_audio_channel.background_music.clone(),
            ).looped();
        } else {
            match state.0 {
                SceneState::InGameClassicMode | SceneState::InGameSurvivalMode => {
                    background_audio_channel.loop_started = false;
                    audio.stop();
                }
                _ => (),
            }
        }
    } else if background_audio_channel.loop_started {
        background_audio_channel.loop_started = false;
        audio.stop();
    }
}

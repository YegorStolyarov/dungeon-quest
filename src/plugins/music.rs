use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};

use crate::resources::setting::Setting;
use crate::scenes::SceneState;

pub struct BackgroundAudioChannel {
    background_music: Handle<AudioSource>,
    channel: AudioChannel,
    loop_started: bool,
    volume: f32,
}

pub fn background_audio_channel_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_audio_channel = BackgroundAudioChannel {
        background_music: asset_server.load("sounds/background_music.ogg"),
        channel: AudioChannel::new("background".to_owned()),
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
            audio.set_volume_in_channel(
                background_audio_channel.volume,
                &background_audio_channel.channel,
            );
            audio.play_looped_in_channel(
                background_audio_channel.background_music.clone(),
                &background_audio_channel.channel,
            );
        } else {
            match *state.current() {
                SceneState::InGameScene => {
                    background_audio_channel.loop_started = false;
                    audio.stop_channel(&background_audio_channel.channel);
                }
                _ => (),
            }
        }
    } else {
        if background_audio_channel.loop_started {
            background_audio_channel.loop_started = false;
            audio.stop_channel(&background_audio_channel.channel);
        }
    }
}

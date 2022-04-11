use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

use crate::plugins::setting::Setting;

pub fn start_background_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    setting: Res<Setting>,
) {
    let background_music = asset_server.load("sounds/background_music.ogg");
    let background_channel = AudioChannel::new("background".to_owned());
    audio.set_volume(0.15);
    audio.set_volume_in_channel(0.15, &background_channel);
    audio.play_looped_in_channel(background_music, &background_channel);

    if setting.get_enable_music() == false {
        stop_background_music(audio);
    } else {
        resume_background_music(audio);
    }
}

pub fn stop_background_music(audio: Res<Audio>) {
    audio.stop();
}

pub fn resume_background_music(audio: Res<Audio>) {
    audio.resume();
}

use bevy::prelude::*;

pub struct IconMaterials {
    pub sound_icon_off: Handle<Image>,
    pub sound_icon_on: Handle<Image>,
    pub sound_icon_hovered: Handle<Image>,
    pub music_icon_off: Handle<Image>,
    pub music_icon_on: Handle<Image>,
    pub music_icon_hovered: Handle<Image>,
    pub home_icon_hovered: Handle<Image>,
    pub home_icon_clicked: Handle<Image>,
    pub home_icon_normal: Handle<Image>,
    pub leaderboard: Handle<Image>,
    pub leaderboard_hovered: Handle<Image>,
    pub restart: Handle<Image>,
    pub restart_hovered: Handle<Image>,
}

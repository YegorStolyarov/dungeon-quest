use bevy::prelude::*;

pub fn cleanup_mouse(mut buttons: ResMut<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        buttons.clear_just_pressed(MouseButton::Left);
    }
}

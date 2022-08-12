use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn system(q: Query<(Entity, &Player)>) {
    // do something
}
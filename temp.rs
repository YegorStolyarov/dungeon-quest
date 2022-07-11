// pub fn book_animation_system(
//     // time: Res<Time>,
//     // texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &Interaction,
//         &HighscoreBook,
//         // &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
// for (interaction, book, sprite, texture_atlas_handle) in query.iter_mut() {
// match interaction {
// Interaction::Clicked => print!("Hello"),
// Interaction::Hovered => print!("Hover"),
// _ => {
// let x = 1;
// }
// }
// }
//         timer.0.tick(time.delta());
//         if timer.0.just_finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             let min_index = 0;
//             let max_index = 5;
//             if sprite.index > max_index || sprite.index < min_index {
//                 sprite.index = min_index;
//             } else {
//                 sprite.index += 1;
//             }
//         }
//     }
// }

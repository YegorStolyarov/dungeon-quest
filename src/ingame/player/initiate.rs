use bevy::prelude::*;

use crate::ingame::materials::InGameMaterials;
use crate::ingame::resources::fixed::data::Data;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::profile::Profile;

const PLAYER_SIZE_WIDTH: f32 = 16.0;
const PLAYER_SIZE_HEIGHT: f32 = 28.0;

pub fn initiate_player(
    mut commands: Commands,
    profile: Res<Profile>,
    data: Res<Data>,
    ingame_materials: Res<InGameMaterials>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let class = profile.hero_class.clone();
    let gender = profile.gender.clone();

    let player = Player::new(class.clone(), data.clone());

    let hero_tileset = ingame_materials
        .heros_materials
        .get_texture(class.clone(), gender);

    let texture_atlas = TextureAtlas::from_grid(
        hero_tileset,
        Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT),
        9,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE_WIDTH * 3.5, PLAYER_SIZE_HEIGHT * 3.5)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.15),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player);
}

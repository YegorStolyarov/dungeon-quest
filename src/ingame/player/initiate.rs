use bevy::prelude::*;

use crate::ingame::materials::InGameMaterials;
use crate::ingame::player::PlayerEntity;
use crate::ingame::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::ingame::resources::data::Data;
use crate::ingame::resources::player::player_animation::PlayerAnimation;
use crate::ingame::resources::player::player_effects::PlayerEffects;
use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::profile::Profile;

const PLAYER_ORIGIN_SIZE_WIDTH: f32 = 16.0;
const PLAYER_ORIGIN_SIZE_HEIGHT: f32 = 28.0;

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
        Vec2::new(PLAYER_ORIGIN_SIZE_WIDTH, PLAYER_ORIGIN_SIZE_HEIGHT),
        9,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let skill = data.get_skill(class);
    let weapon = data.get_weapon(class);

    let entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE_WIDTH, PLAYER_SIZE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.15),
                ..Default::default()
            },
            ..Default::default()
        })
        // .with_children(|parent| => {
            // initiate_weapon(parent, weapon)
        // })
        .insert(player)
        .insert(PlayerAnimation::new())
        .insert(Name::new("Player"))
        .id();

    commands.insert_resource(PlayerEffects::new(data.get_player_effect_information()));
    commands.insert_resource(PlayerSkill::new(skill));
    commands.insert_resource(PlayerEntity { entity });
}

// fn initiate_weapon(player: &mut ChildBuilder, weapon: Weapon) {
    // 
// }

use std::time::Duration;

use bevy::prelude::*;

use crate::components::invinsible_cooldown::InvisibleCooldownComponent;
use crate::components::player::PlayerComponent;
use crate::components::player_animation::PlayerAnimation;
use crate::components::player_list_effects::PlayerListEffectsComponent;
use crate::components::skill::SkillComponent;
use crate::materials::ingame::InGameMaterials;
use crate::plugins::player::PlayerEntity;
use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::resources::game_data::GameData;
use crate::resources::profile::Profile;
use crate::resources::upgrade::upgrade_controller::UpgradeController;

const PLAYER_ORIGIN_SIZE_WIDTH: f32 = 16.0;
const PLAYER_ORIGIN_SIZE_HEIGHT: f32 = 28.0;

pub fn initiate_player(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ingame_materials: Res<InGameMaterials>,
    game_data: Res<GameData>,
    mut commands: Commands,
    profile: Res<Profile>,
) {
    let class = profile.hero_class.clone();
    let gender = profile.gender.clone();

    let skill = game_data.get_skill(class.clone());

    let player = PlayerComponent::new(class.clone(), game_data.clone());

    let hero_tileset = ingame_materials
        .heroes_materials
        .get_texture(class.clone(), gender);

    let texture_atlas = TextureAtlas::from_grid(
        hero_tileset,
        Vec2::new(PLAYER_ORIGIN_SIZE_WIDTH, PLAYER_ORIGIN_SIZE_HEIGHT),
        9,
        1,
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let entity = commands
        .spawn(SpriteSheetBundle {
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
        .insert(player)
        .insert(PlayerAnimation::new())
        .insert(PlayerListEffectsComponent::new(
            game_data.get_player_list_effects_information(),
        ))
        .insert(SkillComponent::new(skill))
        .insert(InvisibleCooldownComponent {
            hurt_duration: Timer::new(Duration::from_secs(0), TimerMode::Once),
            duration: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
        })
        .insert(Name::new("Player"))
        .id();

    commands.insert_resource(UpgradeController::new());
    commands.insert_resource(PlayerEntity { entity });
}

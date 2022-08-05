use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{monster::MonsterComponent, potion::PotionComponent, skill::SkillComponent},
    materials::ingame::InGameMaterials,
    resources::{
        monster::monster_spawn_controller::MonsterSpawnController,
        player::player_dungeon_stats::PlayerDungeonStats, potion::potion_type::PotionType,
        profile::Profile, skill::skill_type::SkillType,
    },
};

pub fn cleanup_monsters(
    mut monsters_query: Query<Entity, With<MonsterComponent>>,
    mut commands: Commands,
) {
    for monster_entity in monsters_query.iter_mut() {
        commands.entity(monster_entity).despawn_recursive();
    }
}

pub fn cleanup_killed_monsters(
    mut monster_spawn_controller: ResMut<MonsterSpawnController>,
    mut monsters_query: Query<(Entity, &Transform, &MonsterComponent)>,
    mut player_skill_query: Query<&mut SkillComponent>,
    ingame_materials: Res<InGameMaterials>,
    mut profile: ResMut<Profile>,
    mut commands: Commands,
) {
    let mut player_skill = player_skill_query.single_mut();
    for (monster_entity, monster_transform, monster) in monsters_query.iter_mut() {
        if monster.current_health_points == 0.0 {
            if player_skill.skill.name == SkillType::Armor {
                player_skill.monster_counter += 1;
            }
            monster_spawn_controller.killed_monsters += 1;
            monster_spawn_controller.alive_monsters -= 1;
            profile.total_killed_monsters += 1;
            commands.entity(monster_entity).despawn_recursive();

            let mut rng = rand::thread_rng();
            let chance = rng.gen_range(0.0..1.0);

            if chance < 0.25 {
                let random = rng.gen_range(0..4);

                let potion_type = match random {
                    0 => PotionType::Heal,
                    1 => PotionType::SpeedUp,
                    2 => PotionType::EvasionUp,
                    _ => PotionType::Focus,
                };

                let texture = match potion_type {
                    PotionType::SpeedUp => ingame_materials.potions_materials.speed_up.clone(),
                    PotionType::Heal => ingame_materials.potions_materials.heal.clone(),
                    PotionType::EvasionUp => ingame_materials.potions_materials.evasion_up.clone(),
                    PotionType::Focus => ingame_materials.potions_materials.focus.clone(),
                };

                let component_name = match potion_type {
                    PotionType::SpeedUp => "SpeedUpPotion",
                    PotionType::Heal => "HealPotion",
                    PotionType::EvasionUp => "EvasionUpPotion",
                    PotionType::Focus => "FocusPotion",
                };

                let x = monster_transform.translation.x;
                let y = monster_transform.translation.y;

                commands
                    .spawn_bundle(SpriteBundle {
                        texture,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(16.0 * 2.0, 16.0 * 2.0)),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.15),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(PotionComponent {
                        potion_type,
                        width: 16.0 * 2.0,
                        height: 16.0 * 2.0,
                    })
                    .insert(Name::new(component_name));
            }
        }
    }
}

pub fn cleanup_monster_after_cleared_room(
    mut monsters_query: Query<Entity, With<MonsterComponent>>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    mut commands: Commands,
) {
    if player_dungeon_stats.is_room_cleared {
        for monster_entity in monsters_query.iter_mut() {
            commands.entity(monster_entity).despawn_recursive();
        }
    }
}

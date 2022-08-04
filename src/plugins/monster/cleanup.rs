use bevy::prelude::*;

use crate::{
    components::{monster::MonsterComponent, skill::SkillComponent},
    resources::{
        monster::monster_spawn_controller::MonsterSpawnController,
        player::player_dungeon_stats::PlayerDungeonStats, profile::Profile,
        skill::skill_type::SkillType,
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
    mut monsters_query: Query<(Entity, &MonsterComponent)>,
    mut monster_spawn_controller: ResMut<MonsterSpawnController>,
    mut player_skill_query: Query<&mut SkillComponent>,
    mut profile: ResMut<Profile>,
    mut commands: Commands,
) {
    let mut player_skill = player_skill_query.single_mut();
    for (monster_entity, monster) in monsters_query.iter_mut() {
        if monster.current_health_points == 0.0 {
            if player_skill.skill.name == SkillType::Armor {
                player_skill.monster_counter += 1;
            }
            monster_spawn_controller.killed_monsters += 1;
            monster_spawn_controller.alive_monsters -= 1;
            profile.total_killed_monsters += 1;
            commands.entity(monster_entity).despawn_recursive();
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

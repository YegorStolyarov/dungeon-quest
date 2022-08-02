use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::components::skill::SkillComponent;
use crate::resources::dungeon::Dungeon;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::resources::skill::skill_type::SkillType;

pub fn unlock_room_cheat(
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut dungeon: ResMut<Dungeon>,
) {
    if keyboard_input.pressed(KeyCode::C) {
        let current_position = dungeon.current_floor.current_position;
        dungeon
            .current_floor
            .cleared_positions
            .insert(current_position, 1);
        player_dungeon_stats.is_room_cleared = true;
        keyboard_input.reset(KeyCode::C);
    }
}

pub fn knight_skill_cheat(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_skill_query: Query<&mut SkillComponent>,
) {
    if keyboard_input.pressed(KeyCode::M) {
        let mut player_skill = player_skill_query.single_mut();
        if player_skill.skill.name == SkillType::Armor {
            player_skill.monster_counter += 1;
        }
        keyboard_input.reset(KeyCode::M);
    }
}

pub fn damage_player_cheat(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut PlayerComponent>,
) {
    if keyboard_input.pressed(KeyCode::N) {
        let mut player = player_query.single_mut();
        player.current_health_points -= 1.0;
        keyboard_input.reset(KeyCode::N);
    }
}

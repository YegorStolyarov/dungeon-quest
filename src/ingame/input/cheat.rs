use bevy::prelude::*;

use crate::ingame::resources::dungeon::Dungeon;
use crate::ingame::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::skill::skill_type::SkillType;

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
    mut player_skill: ResMut<PlayerSkill>,
) {
    if keyboard_input.pressed(KeyCode::M) {
        if player_skill.skill.name == SkillType::Armor {
            let value = player_skill.require_monsters.clone();
            player_skill.monster_counter = value;
        }
        keyboard_input.reset(KeyCode::M);
    }
}

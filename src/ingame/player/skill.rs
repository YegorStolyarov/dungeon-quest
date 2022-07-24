use bevy::prelude::*;

use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::skill::skill_type::SkillType;

pub fn cooldown(time: Res<Time>, mut player_skill: ResMut<PlayerSkill>) {
    if !player_skill.cooldown.finished() {
        player_skill.cooldown.tick(time.delta());
    }
}

pub fn duration(time: Res<Time>, mut player_skill: ResMut<PlayerSkill>) {
    if !player_skill.cooldown.finished() {
        player_skill.duration.tick(time.delta());
    }
}

pub fn knight_skill(mut player_skill: ResMut<PlayerSkill>, mut player_query: Query<&mut Player>) {
    if player_skill.skill.name == SkillType::Armor {
        if player_skill.require_monsters == player_skill.monster_counter {
            let mut player = player_query.single_mut();

            if player.current_health_points < player.max_health_points {
                let new_health_points = player.current_health_points + 1.0;

                player.current_health_points = if new_health_points > player.max_health_points {
                    player.max_health_points
                } else {
                    new_health_points
                };

                player_skill.monster_counter = 0;

                dbg!(player_skill.monster_counter);
            }
        }
    }
}

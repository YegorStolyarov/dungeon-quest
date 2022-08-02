use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::components::skill::SkillComponent;
use crate::resources::skill::skill_type::SkillType;

pub fn cooldown(time: Res<Time>, mut player_skill_query: Query<&mut SkillComponent>) {
    let mut player_skill = player_skill_query.single_mut();
    if !player_skill.cooldown.finished() {
        player_skill.cooldown.tick(time.delta());
    }
}

pub fn duration(time: Res<Time>, mut player_skill_query: Query<&mut SkillComponent>) {
    let mut player_skill = player_skill_query.single_mut();
    if !player_skill.cooldown.finished() {
        player_skill.duration.tick(time.delta());
    }
}

pub fn knight_skill(mut knight_query: Query<(&mut PlayerComponent, &mut SkillComponent)>) {
    let (mut player, mut skill_component) = knight_query.single_mut();
    if skill_component.skill.name == SkillType::Armor {
        if skill_component.require_monsters == skill_component.monster_counter {
            if player.current_health_points < player.max_health_points {
                let new_health_points = player.current_health_points + 1.0;
                player.current_health_points = if new_health_points > player.max_health_points {
                    player.max_health_points
                } else {
                    new_health_points
                };
                skill_component.monster_counter = 0;
            }
        }
    }
}

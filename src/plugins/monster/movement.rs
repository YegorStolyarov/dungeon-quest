use bevy::prelude::*;

use crate::{
    components::{
        monster::MonsterComponent, monster_animation::MonsterAnimationComponent,
        player::PlayerComponent, skill::SkillComponent,
    },
    resources::{animation_state::AnimationState, skill::skill_type::SkillType},
};

pub fn move_to_player(
    mut set: ParamSet<(
        Query<(&Transform, &SkillComponent), With<PlayerComponent>>,
        Query<(
            &MonsterComponent,
            &mut MonsterAnimationComponent,
            &mut Transform,
        )>,
    )>,
) {
    let mut should_move = true;
    let mut target = Vec3::new(0.0, 0.0, 0.15);

    for (transform, skill_component) in set.p0().iter() {
        if skill_component.skill.name == SkillType::TimeToHunt {
            if !skill_component.duration.finished() {
                should_move = false;
            }
        }
        target.x = transform.translation.x;
        target.y = transform.translation.y;
    }

    for (monster_component, mut monster_animation_component, mut transform) in set.p1().iter_mut() {
        if !should_move {
            monster_animation_component.animation_state = AnimationState::Idle;
        } else {
            monster_animation_component.animation_state = AnimationState::Moving;
            let dir = (target - transform.translation).normalize();
            let mut new_position = transform.translation + dir * monster_component.speed * 0.2;
            new_position.z = 0.15;
            transform.translation = new_position;
        }
    }
}

pub fn change_direction(
    mut set: ParamSet<(
        Query<&Transform, With<PlayerComponent>>,
        Query<&mut Transform, With<MonsterComponent>>,
    )>,
) {
    let mut target_x = 0.0;
    for transform in set.p0().iter() {
        target_x = transform.translation.x;
    }

    for mut transform in set.p1().iter_mut() {
        let delta_x = transform.translation.x - target_x;
        if delta_x < 0.0 {
            transform.rotation = Quat::default();
        } else if delta_x > 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
    }
}

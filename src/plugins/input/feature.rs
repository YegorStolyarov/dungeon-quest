use bevy::prelude::*;
use std::time::Duration;

use crate::components::player::PlayerComponent;
use crate::components::skill::SkillComponent;
use crate::components::weapon::WeaponComponent;
use crate::resources::animation_state::AnimationState;
use crate::resources::player::player_animation::PlayerAnimation;
use crate::resources::skill::skill_type::SkillType;
use crate::resources::weapon::attack_type::AttackType;
use crate::resources::weapon::weapon_type::WeaponType;
use crate::scenes::SceneState;

pub fn pause(mut keyboard_input: ResMut<Input<KeyCode>>, mut state: ResMut<State<SceneState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state.push(SceneState::PauseScene).unwrap();
        keyboard_input.reset(KeyCode::Escape);
    }
}

pub fn use_skill(
    mut player_query: Query<(&mut PlayerComponent, &mut SkillComponent)>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    let (mut player, mut player_skill) = player_query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        if player_skill.cooldown.finished() {
            match player_skill.skill.name {
                SkillType::Armor => {}
                SkillType::Thunderstorm => {
                    let cooldown = player_skill.skill.cooldown.unwrap() as u64;
                    player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                    todo!("With Monster");
                }
                SkillType::TimeToHunt => {
                    let skill = player_skill.skill.clone();
                    let duration = skill.duration.unwrap() as u64;
                    player_skill.duration = Timer::new(Duration::from_secs(duration), false);
                    let cooldown = skill.cooldown.unwrap() as u64;
                    player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                }
                SkillType::AnimalInstinct => {
                    let skill = player_skill.skill.clone();
                    let require_health = skill.require_health_points.unwrap();
                    if player.current_health_points > require_health {
                        player.current_health_points -= require_health;
                        let duration = skill.duration.unwrap() as u64;
                        player_skill.duration = Timer::new(Duration::from_secs(duration), false);
                        let cooldown = skill.cooldown.unwrap() as u64;
                        player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                    }
                }
            }
        }
        keyboard_input.reset(KeyCode::Space);
    }
}

pub fn use_mouse(
    mut weapon_query: Query<&mut WeaponComponent>,
    mut buttons: ResMut<Input<MouseButton>>,
    player_animation_query: Query<&PlayerAnimation>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mut weapon_component = weapon_query.single_mut();
        let player_animation = player_animation_query.single();

        match weapon_component.attack_type {
            AttackType::Swing => {
                // if weapon_component.attack_duration.finished() {
                // weapon_component.attack_duration = Timer::new(Duration::from_secs(1), false);
                // }
            }
            AttackType::Shoot => {
                if weapon_component.cooldown.finished() {
                    if weapon_component.name == WeaponType::Bow {
                        if player_animation.animation_state == AnimationState::Idle {
                            weapon_component.spawn_bullet = true;
                            weapon_component.cooldown = Timer::new(
                                Duration::from_secs(weapon_component.cooldown_second),
                                false,
                            );
                        }
                    } else {
                        weapon_component.spawn_bullet = true;
                        weapon_component.cooldown = Timer::new(
                            Duration::from_secs(weapon_component.cooldown_second),
                            false,
                        );
                    }
                }
            }
            AttackType::Throw => {}
        }
        buttons.clear_just_pressed(MouseButton::Left);
    }
}

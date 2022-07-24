use bevy::prelude::*;
use std::time::Duration;

use crate::ingame::resources::player::player_skill::PlayerSkill;
use crate::ingame::resources::player::Player;
use crate::ingame::resources::skill::skill_type::SkillType;
use crate::scenes::SceneState;

pub fn pause(mut keyboard_input: ResMut<Input<KeyCode>>, mut state: ResMut<State<SceneState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        state.push(SceneState::PauseScene).unwrap();
        keyboard_input.reset(KeyCode::Escape);
    }
}

pub fn use_skill(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut player_skill: ResMut<PlayerSkill>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if player_skill.cooldown.finished() {
            let mut player = player_query.single_mut();

            match player_skill.skill.name {
                SkillType::Armor => {}
                SkillType::Thunderstorm => {
                    let cooldown = player_skill.skill.cooldown.unwrap();
                    player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                    todo!("With Monster");
                }
                SkillType::TimeToHunt => {
                    let skill = player_skill.skill.clone();

                    let duration = skill.duration.unwrap();
                    let cooldown = skill.cooldown.unwrap();

                    player_skill.duration = Timer::new(Duration::from_secs(duration), false);
                    player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                }
                SkillType::AnimalInstinct => {
                    let skill = player_skill.skill.clone();
                    let require_health = skill.require_health_points.unwrap();
                    if player.current_health_points > require_health {
                        player.current_health_points -= require_health;
                        let duration = skill.duration.unwrap();
                        let cooldown = skill.cooldown.unwrap();

                        player_skill.duration = Timer::new(Duration::from_secs(duration), false);
                        player_skill.cooldown = Timer::new(Duration::from_secs(cooldown), false);
                    }
                }
            }
        }
    }
}

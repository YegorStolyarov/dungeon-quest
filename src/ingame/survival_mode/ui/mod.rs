use bevy::prelude::*;
use std::time::Duration;

use crate::ingame::resources::dungeon::wave::Wave;
use crate::resources::dictionary::Dictionary;
use crate::resources::materials::Materials;
use crate::scenes::SceneState;

pub struct SurvivalModeUIPlugin;

#[derive(Component)]
pub struct CenterText {
    pub timer: Timer,
}

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct WaveCountDownText;

struct SurvivalModeUIData {
    pub user_interface_root: Entity,
}

impl Plugin for SurvivalModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::InGameSurvivalMode).with_system(setup));

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameSurvivalMode)
                .with_system(center_text_handle_system)
                .with_system(wave_text_handle_system)
                .with_system(wave_countdown_text_handle_system),
        );

        app.add_system_set(
            SystemSet::on_resume(SceneState::InGameSurvivalMode).with_system(reset_center_text),
        );

        app.add_system_set(SystemSet::on_exit(SceneState::InGameSurvivalMode).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, materials: Res<Materials>, dictionary: Res<Dictionary>) {
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            center_text(parent, &materials, &dictionary);
            wave_text(parent, &materials, &dictionary);
            wave_countdown_text(parent, &materials, &dictionary);
        })
        .insert(Name::new("SurvivalModeUI"))
        .id();

    commands.insert_resource(SurvivalModeUIData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, survival_mode_ui_data: Res<SurvivalModeUIData>) {
    commands
        .entity(survival_mode_ui_data.user_interface_root)
        .despawn_recursive();
}

fn center_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let value = format!("{} {}", glossary.ingame_text.wave.clone(), 1);

    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        text: Text::with_section(
            value,
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
    .insert(CenterText {
        timer: Timer::new(Duration::from_secs(1), false),
    })
    .insert(Name::new("CenterText"));
}

fn center_text_handle_system(
    mut text_query: Query<(&mut CenterText, &mut Text, &mut Visibility)>,
    dictionary: Res<Dictionary>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let (mut center_text, mut text, mut visibility) = text_query.single_mut();
    center_text.timer.tick(time.delta());
    if center_text.timer.finished() {
        visibility.is_visible = false;
    } else {
        let glossary = dictionary.get_glossary();
        let current_floor_index = wave.wave_number;

        let value = format!(
            "{} {}",
            glossary.ingame_text.wave.clone(),
            current_floor_index
        );

        text.sections[0].value = value;
        visibility.is_visible = true;
    }
}

fn wave_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());

    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                right: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "1",
            TextStyle {
                font: font.clone(),
                font_size: 35.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
    .insert(WaveText)
    .insert(Name::new("WaveText"));
}

fn wave_text_handle_system(mut text_query: Query<&mut Text, With<WaveText>>, wave: Res<Wave>) {
    let mut text = text_query.single_mut();

    if wave.is_changed() {
        text.sections[0].value = wave.wave_number.to_string();
    }
}

fn wave_countdown_text(root: &mut ChildBuilder, materials: &Materials, dictionary: &Dictionary) {
    let font = materials.get_font(dictionary.get_current_language());

    root.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(35.0),
                right: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 35.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
    .insert(WaveCountDownText)
    .insert(Name::new("WaveCountDownText"));
}

fn wave_countdown_text_handle_system(
    mut wave_countdown_text_query: Query<&mut Text, With<WaveCountDownText>>,
    wave: Res<Wave>,
) {
    let timer = wave.timer.clone();
    let elapsed_seconds = timer.elapsed_secs();

    let full_time_seconds = wave.wave_duration;

    let seconds_left = full_time_seconds - elapsed_seconds as u64;

    let seconds = seconds_left % 60;
    let formated_seconds = match seconds {
        0..=9 => format!("0{}", seconds),
        _ => format!("{}", seconds),
    };

    let minutes = seconds_left / 60 % 60;
    let formated_minutes = match minutes {
        0..=9 => format!("0{}", minutes),
        _ => format!("{}", minutes),
    };

    let value = format!("{}:{}", formated_minutes, formated_seconds);

    let mut wave_countdown_text = wave_countdown_text_query.single_mut();

    wave_countdown_text.sections[0].value = value;
}

fn reset_center_text(mut center_text_query: Query<&mut CenterText>, wave: Res<Wave>) {
    if wave.is_changed() {
        let mut center_text = center_text_query.single_mut();
        center_text.timer = Timer::new(Duration::from_secs(1), false);
    }
}

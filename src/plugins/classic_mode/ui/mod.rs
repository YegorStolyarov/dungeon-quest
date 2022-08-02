use bevy::prelude::*;
use std::time::Duration;

use crate::materials::font::FontMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::scenes::SceneState;

pub struct ClassicModeUIPlugin;

#[derive(Component)]
pub struct CenterTextComponent {
    pub timer: Timer,
}

#[derive(Component)]
struct FloorTextComponent;

struct ClassicModeUIData {
    pub user_interface_root: Entity,
}

impl Plugin for ClassicModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::InGameClassicMode).with_system(setup));

        app.add_system_set(
            SystemSet::on_update(SceneState::InGameClassicMode)
                .with_system(center_text_handle_system)
                .with_system(top_right_conner_text_handle_system),
        );

        app.add_system_set(SystemSet::on_exit(SceneState::InGameClassicMode).with_system(cleanup));
    }
}

fn setup(mut commands: Commands, font_materials: Res<FontMaterials>, dictionary: Res<Dictionary>) {
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
            center_text(parent, &font_materials, &dictionary);
            floor_text(parent, &font_materials, &dictionary);
        })
        .insert(Name::new("ClassicModeUI"))
        .id();

    commands.insert_resource(ClassicModeUIData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, classic_mode_ui_data: Res<ClassicModeUIData>) {
    commands
        .entity(classic_mode_ui_data.user_interface_root)
        .despawn_recursive();
}

fn center_text(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let value = format!("{} {}", glossary.ingame_text.floor.clone(), 1);

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
    .insert(CenterTextComponent {
        timer: Timer::new(Duration::from_secs(1), false),
    })
    .insert(Name::new("CenterText"));
}

fn center_text_handle_system(
    mut text_query: Query<(&mut CenterTextComponent, &mut Text, &mut Visibility)>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    dictionary: Res<Dictionary>,
    time: Res<Time>,
) {
    let (mut center_text, mut text, mut visibility) = text_query.single_mut();
    center_text.timer.tick(time.delta());

    if center_text.timer.finished() {
        visibility.is_visible = false;
    } else {
        let glossary = dictionary.get_glossary();
        let current_floor_index = player_dungeon_stats.current_floor_index;

        let value = format!(
            "{} {}",
            glossary.ingame_text.floor.clone(),
            current_floor_index + 1
        );

        text.sections[0].value = value;
        visibility.is_visible = true;
    }
}

fn floor_text(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
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
    .insert(FloorTextComponent)
    .insert(Name::new("FloorTextComponent"));
}

fn top_right_conner_text_handle_system(
    mut text_query: Query<&mut Text, With<FloorTextComponent>>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
) {
    let mut text = text_query.single_mut();

    if player_dungeon_stats.is_changed() {
        text.sections[0].value = (player_dungeon_stats.current_floor_index + 1).to_string();
    }
}

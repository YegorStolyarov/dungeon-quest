use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use std::slice::Iter;

use crate::config::*;
use crate::resources::dictionary::Dictionary;
use crate::resources::tile_size::TileSize;
use crate::scenes::SceneState;

const BOOK_TILE_IMAGE: &str = "images/gui/book.png";
const BOOK_TILE_SIZE: TileSize = TileSize {
    width: 190.0,
    height: 160.0,
};

#[derive(Component, Copy, Clone)]
enum HighscoreSceneButton {
    Return,
    Open,
    Next,
    Previous,
}

impl HighscoreSceneButton {
    pub fn iterator() -> Iter<'static, HighscoreSceneButton> {
        static HIGHSCORE_SCENE_BUTTONS: [HighscoreSceneButton; 4] = [
            HighscoreSceneButton::Return,
            HighscoreSceneButton::Open,
            HighscoreSceneButton::Next,
            HighscoreSceneButton::Previous,
        ];
        HIGHSCORE_SCENE_BUTTONS.iter()
    }
}

const HIGHSCORE_SCENE_BUTTON_POSITIONS: [Rect<Val>; 4] = [
    Rect {
        left: Val::Px(20.0),
        top: Val::Px(20.0),
        right: Val::Auto,
        bottom: Val::Auto,
    },
    Rect {
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        right: Val::Auto,
        bottom: Val::Auto,
    },
    Rect {
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        bottom: Val::Auto,
        right: Val::Auto,
    },
    Rect {
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        right: Val::Auto,
        bottom: Val::Auto,
    },
];

const HIGHSCORE_SCENE_BUTTON_SIZE: Size<Val> = Size {
    width: Val::Px(50.0),
    height: Val::Px(50.0),
};

struct HighscoreSceneData {
    user_interface_root: Entity,
    background: Entity,
    book: Entity,
}

#[derive(Component)]
pub struct HighscoreBook;

#[derive(Component)]
pub struct AnimationTimer(Timer);

pub struct HighscoreScenePlugin;

impl Plugin for HighscoreScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::HighscoreScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::HighscoreScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::HighscoreScene).with_system(cleanup));
    }
}

fn cleanup(mut commands: Commands, highscore_scene_data: Res<HighscoreSceneData>) {
    commands
        .entity(highscore_scene_data.background)
        .despawn_recursive();

    commands
        .entity(highscore_scene_data.book)
        .despawn_recursive();

    commands
        .entity(highscore_scene_data.user_interface_root)
        .despawn_recursive();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dictionary: Res<Dictionary>,
) {
    dbg!("OUTHERE");

    // background
    let background = commands
        .spawn_bundle(SpriteBundle {
            // texture: materials.background.clone(),
            ..Default::default()
        })
        .id();

    // book
    let texture_handle = asset_server.load(BOOK_TILE_IMAGE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BOOK_TILE_SIZE.width, BOOK_TILE_SIZE.width),
        7,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let book = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(-25.0, -30.0, 1.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(BOOK_TILE_SIZE.width, BOOK_TILE_SIZE.height)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HighscoreBook)
        .id();

    // user interface root
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| buttons(parent, &asset_server, dictionary))
        .id();

    commands.insert_resource(HighscoreSceneData {
        user_interface_root,
        background,
        book,
    });
}

fn buttons(root: &mut ChildBuilder, asset_server: &Res<AssetServer>, dictionary: Res<Dictionary>) {
    for (index, button) in HighscoreSceneButton::iterator().enumerate() {
        // match image

        root.spawn_bundle(ButtonBundle {
            style: Style {
                position: HIGHSCORE_SCENE_BUTTON_POSITIONS[index],
                size: HIGHSCORE_SCENE_BUTTON_SIZE,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image: UiImage(asset_server.load("icons/home_icon_normal.png")),
            ..Default::default()
        })
        .insert(button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &HighscoreSceneButton, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            HighscoreSceneButton::Return => match *interaction {
                Interaction::None => ui_image.0 = asset_server.load("icons/home_icon_normal.png"),
                Interaction::Hovered => {
                    ui_image.0 = asset_server.load("icons/home_icon_hovered.png")
                }
                Interaction::Clicked => {
                    ui_image.0 = asset_server.load("icons/home_icon_clicked.png");
                    state
                        .set(SceneState::MainMenuScene)
                        .expect("Couldn't switch state to Main Menu Screen");
                }
            },
            _ => print!("A"),
        }
    }
}

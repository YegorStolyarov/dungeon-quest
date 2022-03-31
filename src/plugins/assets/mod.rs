use bevy::prelude::*;

#[derive(Component)]
pub struct UiAsset(pub Handle<TextureAtlas>);

pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("images/menu_buttons.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::splat(9.0),
        1,
        2,
        Vec2::splat(2.0),
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(UiAsset(atlas_handle));
}

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player_two(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, z),
            texture: asset_server.load("sprites/")
        },
        Player_Two
    ));
}
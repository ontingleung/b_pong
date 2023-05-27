use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::player_two::components::*;

pub const PLAYER_TWO_SIZE: f32 = 20.0;

pub fn spawn_player_two(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - PLAYER_TWO_SIZE, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player_two.png"),
            ..default()
        },
        PlayerTwo {}
    ));
}
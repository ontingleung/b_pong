use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::player_one::components::*;

pub const PLAYER_ONE_SIZE: f32 = 20.0;
pub const PLAYER_SPEED: f32 = 600.0;

pub fn spawn_player_one(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(PLAYER_ONE_SIZE, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player_one.png"),
            ..default()
        },
        PlayerOne {}
    ));
}

pub fn player_one_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerOne>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::player_two::components::*;

pub const PLAYER_TWO_SIZE: f32 = 20.0;
pub const PLAYER_SPEED: f32 = 600.0;

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

pub fn player_two_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerTwo>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

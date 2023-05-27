use crate::game::ball::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const BALL_SIZE: f32 = 64.0;
pub const BALL_SPEED: f32 = 250.0;

pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_red.png"),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),       
        },
    ));
}

pub fn ball_movement(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    for (mut transform, ball) in ball_query.iter_mut() {
        let direction = Vec3::new(ball.direction.x, ball.direction.y, 0.0);
        transform.translation += direction * BALL_SPEED * time.delta_seconds();
    }
}

pub fn confine_ball_movement(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;
    let x_min = 0.0 + half_ball_size;
    let x_max = window.width() - half_ball_size;
    let y_min = 0.0 + half_ball_size;
    let y_max = window.height() - half_ball_size;

    for mut transform in ball_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the ball y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
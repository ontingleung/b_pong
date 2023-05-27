use crate::game::ball::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;


pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprite/"),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),       
        },
    ));
}
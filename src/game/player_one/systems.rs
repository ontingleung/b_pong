use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player_one(
    mut commands: Commands,
    window_query: Query<&Window, &PrimaryWindow>,
    asset_server: Res<AssetServer>,
) {
    
}
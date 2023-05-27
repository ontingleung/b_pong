mod components;
mod systems;

use bevy::prelude::*;
use systems::*;
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_ball)
            .add_system(ball_movement)
            .add_system(confine_ball_movement);
    }
}
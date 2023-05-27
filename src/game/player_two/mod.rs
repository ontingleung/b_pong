mod systems;
mod components;

use bevy::prelude::*;
use systems::*;

pub struct PlayerTwoPlugin;

impl Plugin for PlayerTwoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player_two)
            .add_system(player_two_movement);
    }
}
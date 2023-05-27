mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PlayerOnePlugin;

impl Plugin for PlayerOnePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player_one);
    }
}


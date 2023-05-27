mod game;
mod systems;


use systems::*;
use bevy::prelude::*;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .run()
}



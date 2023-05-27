mod player_one;
mod player_two;
mod ball;

use bevy::prelude::*;

use player_one::PlayerOnePlugin;
use player_two::PlayerTwoPlugin;
use ball::BallPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerOnePlugin)
            .add_plugin(PlayerTwoPlugin)
            .add_plugin(BallPlugin);
    }
}
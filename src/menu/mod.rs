use bevy::prelude::*;

use resources::GameState;
use systems::startup;

pub mod resources;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Startup, startup);
    }
}

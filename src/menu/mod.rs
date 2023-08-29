pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::GameState;
use systems::render_loop;
use systems::startup;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Startup, startup)
            .add_systems(Update, render_loop.run_if(in_state(AppState::Menu)));
    }
}

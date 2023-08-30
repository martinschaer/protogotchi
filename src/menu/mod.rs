mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::MenuState;
use systems::{navigation, on_enter, render_loop, startup};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuState>()
            .add_systems(Startup, startup)
            .add_systems(OnEnter(AppState::Menu), on_enter)
            .add_systems(
                Update,
                (navigation, render_loop).run_if(in_state(AppState::Menu)),
            );
    }
}

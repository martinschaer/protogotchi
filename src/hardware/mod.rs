use bevy::prelude::*;

use resources::Hardware;
use systems::{buttons, render_loop, startup};

mod resources;
mod systems;

pub struct HardwarePlugin;

impl Plugin for HardwarePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hardware>()
            .add_systems(Startup, startup)
            .add_systems(Update, buttons)
            .add_systems(Update, render_loop);
    }
}

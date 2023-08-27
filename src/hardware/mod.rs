use bevy::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;

use resources::Hardware;
use systems::{render_loop, startup};

mod resources;
mod systems;

pub struct HardwarePlugin;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
// palette
//
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

impl Plugin for HardwarePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hardware>()
            .add_systems(Startup, startup)
            .add_systems(Update, render_loop);
    }
}

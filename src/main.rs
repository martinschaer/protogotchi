pub mod hardware;
pub mod menu;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use embedded_graphics::pixelcolor::Rgb565;
use std::time::Duration;

// use hardware::resources::Hardware;
use hardware::HardwarePlugin;
use menu::MenuPlugin;

pub const W_SIZE: usize = 320;
pub const H_SIZE: usize = 240;

// bg
pub const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
// palette
//
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(HardwarePlugin)
        .add_plugins(MenuPlugin)
        .run();
}

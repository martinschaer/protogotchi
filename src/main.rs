#[cfg(target_os = "linux")]
pub mod hardware;

#[cfg(target_os = "macos")]
pub mod sim;

pub mod menu;

use bevy::prelude::*;

#[cfg(target_os = "linux")]
use bevy::app::ScheduleRunnerPlugin;

#[cfg(target_os = "linux")]
use std::time::Duration;

#[cfg(target_os = "linux")]
use hardware::HardwarePlugin;

#[cfg(target_os = "macos")]
use sim::SimPlugin;

use embedded_graphics::pixelcolor::Rgb565;

use menu::MenuPlugin;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
// palette
//
// const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

#[derive(Resource)]
pub struct Render {
    data: [Rgb565; W_SIZE * H_SIZE],
}

impl Default for Render {
    fn default() -> Self {
        let data: [Rgb565; W_SIZE * H_SIZE] = [COLOR_BLUE; W_SIZE * H_SIZE];
        Render { data }
    }
}

#[cfg(target_os = "linux")]
fn main() {
    App::new()
        .init_resource::<Render>()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(HardwarePlugin)
        .add_plugins(MenuPlugin)
        .run();
}

#[cfg(target_os = "macos")]
fn main() {
    App::new()
        .init_resource::<Render>()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimPlugin)
        .add_plugins(MenuPlugin)
        .run();
}

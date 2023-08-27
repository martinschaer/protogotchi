#[cfg(target_os = "linux")]
pub mod hardware;

#[cfg(target_os = "macos")]
mod simulator;

pub mod menu;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

#[cfg(target_os = "linux")]
use hardware::HardwarePlugin;

#[cfg(target_os = "macos")]
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

#[cfg(target_os = "macos")]
use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};

#[cfg(target_os = "macos")]
use simulator::SimulatorPlugin;

use menu::MenuPlugin;

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "macos")]
#[derive(Resource)]
pub struct Sim {
    display: SimulatorDisplay<Rgb565>,
}

#[cfg(target_os = "macos")]
impl Default for Sim {
    fn default() -> Self {
        let display: SimulatorDisplay<Rgb565> =
            SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
        let output_settings = OutputSettingsBuilder::new().build();
        Window::new("Hello World", &output_settings).show_static(&display);
        Sim { display }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    App::new()
        .init_resource::<Sim>()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(SimulatorPlugin)
        .add_plugins(MenuPlugin)
        .run();
}

#[cfg(target_os = "linux")]
pub mod hardware;

pub mod menu;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

#[cfg(target_os = "linux")]
use hardware::HardwarePlugin;

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
fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(MenuPlugin)
        .run();
}

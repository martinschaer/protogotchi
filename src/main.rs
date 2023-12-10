#[cfg(all(target_os = "linux", target_arch = "arm"))]
mod hardware;

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    all(target_os = "linux", not(target_arch = "arm"))
))]
mod sim;

mod input;
mod menu;
mod plugins;
mod settings;
mod splash;
mod systems;

use bevy::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use std::collections::HashMap;

#[cfg(target_os = "macos")]
use bevy_pixels::prelude::*;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use bevy::app::ScheduleRunnerPlugin;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use std::time::Duration;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use hardware::HardwarePlugin;

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    all(target_os = "linux", not(target_arch = "arm"))
))]
use sim::SimPlugin;

use input::InputPlugin;
use menu::MenuPlugin;
use plugins::select::SelectPlugin;
use settings::{wifi::WifiPlugin, SettingsPlugin};
use splash::SplashPlugin;

// bg
// const COLOR_888_BG: Rgb888 = Rgb888::new(0xef, 0xfa, 0xfa);
const COLOR_BG: Rgb565 = Rgb565::new(0b11101, 0b111110, 0b11111);
// text
const COLOR_FG: Rgb565 = Rgb565::new(0b00001, 0b001001, 0b00100);
// primary
const COLOR_PRIMARY: Rgb565 = Rgb565::new(0b0, 0b011110, 0b01110);
// const COLOR_SECONDARY: Rgb565 = Rgb565::from(Rgb888::new(0xd4, 0xf2, 0xf1));
// const COLOR_ACCENT: Rgb565 = Rgb565::from(Rgb888::new(0xc2, 0x38, 0x3f));

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Settings,
    Wifi,
    Input,
}

#[derive(Resource, Default)]
pub struct CurrentRouteState {
    pub params: Vec<String>,
}

#[derive(Resource)]
pub struct Render {
    pub data: [Rgb565; W_SIZE * H_SIZE],
    // pub route: String,
    pub button_a_pressed: bool,
    pub button_b_pressed: bool,
    pub button_x_pressed: bool,
    pub button_y_pressed: bool,
}

impl Default for Render {
    fn default() -> Self {
        let data: [Rgb565; W_SIZE * H_SIZE] = [COLOR_BG; W_SIZE * H_SIZE];
        Render {
            data,
            // route: String::from("/"),
            button_a_pressed: false,
            button_b_pressed: false,
            button_x_pressed: false,
            button_y_pressed: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct DB {
    pub records: HashMap<String, String>,
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
fn main() {
    App::new()
        .init_resource::<Render>()
        // Bevy Plugins
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        // My Plugins
        .add_plugins(InputPlugin)
        .add_plugins(SelectPlugin)
        .add_plugins(HardwarePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(WifiPlugin)
        // Systems
        // Run
        .run();
}

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    all(target_os = "linux", not(target_arch = "arm"))
))]
fn main() {
    App::new()
        .init_resource::<Render>()
        .init_resource::<CurrentRouteState>()
        .init_resource::<DB>()
        .add_state::<AppState>()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // My Plugins
        .add_plugins(InputPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SelectPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(WifiPlugin)
        // Systems
        .add_systems(Update, bevy::window::close_on_esc)
        // Run
        .run();
}

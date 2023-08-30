#[cfg(target_os = "linux")]
mod hardware;

#[cfg(target_os = "macos")]
mod sim;

mod menu;
mod settings;
mod splash;
mod systems;

use bevy::prelude::*;

#[cfg(target_os = "macos")]
use bevy_pixels::prelude::*;

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
use settings::SettingsPlugin;
use splash::SplashPlugin;
use systems::transition_to_splash_state;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
// palette
//
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

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
        let data: [Rgb565; W_SIZE * H_SIZE] = [COLOR_BLUE; W_SIZE * H_SIZE];
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

#[cfg(target_os = "linux")]
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
        .add_plugins(HardwarePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(SplashPlugin)
        // Systems
        .add_systems(Update, transition_to_splash_state)
        // Run
        .run();
}

#[cfg(target_os = "macos")]
fn main() {
    App::new()
        .init_resource::<Render>()
        .add_state::<AppState>()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // My Plugins
        .add_plugins(MenuPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(SplashPlugin)
        // Systems
        .add_systems(Update, transition_to_splash_state)
        .add_systems(Update, bevy::window::close_on_esc)
        // Run
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Settings,
}

use bevy::prelude::*;
use embedded_graphics::draw_target::DrawTarget;

pub mod resources;

use crate::COLOR_BLUE;
use resources::Hardware;

pub struct HardwarePlugin;

fn startup(mut hardware: ResMut<Hardware>) {
    hardware.led_r.set_pwm_frequency(50., 1.).unwrap();
    hardware.led_g.set_pwm_frequency(50., 1.).unwrap();
    hardware.led_b.set_pwm_frequency(50., 1.).unwrap();

    // Turn on backlight
    // hardware.backlight.set_low();
    // sleep(Duration::from_millis(150));
    hardware.backlight.set_high();

    // Clear the display initially
    // hardware.display.clear(COLOR_BLUE).unwrap();
    hardware.display.lock().unwrap().clear(COLOR_BLUE).unwrap();
}

impl Plugin for HardwarePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hardware>()
            .add_systems(Startup, startup);
    }
}

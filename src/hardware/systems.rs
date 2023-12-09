use bevy::prelude::*;
use embedded_graphics::{prelude::*, primitives::Rectangle};
use embedded_graphics_framebuf::FrameBuf;

use super::resources::Hardware;
use crate::{Render, H_SIZE, W_SIZE};

pub fn startup(mut hardware: ResMut<Hardware>) {
    hardware.led_r.set_pwm_frequency(50., 1.).unwrap();
    hardware.led_g.set_pwm_frequency(50., 1.).unwrap();
    hardware.led_b.set_pwm_frequency(50., 1.).unwrap();

    // Turn on backlight
    // hardware.backlight.set_low();
    // sleep(Duration::from_millis(150));
    hardware.backlight.set_high();
}

pub fn render_loop(time: Res<Time>, mut hardware: ResMut<Hardware>, mut render: ResMut<Render>) {
    let elapsed = time.elapsed_seconds_f64();

    // led
    let y = (elapsed.sin() + 1.) * 0.5;
    let stepped_y = (y * 100.).round();
    hardware
        .led_r
        .set_pwm_frequency(50., stepped_y / 100.)
        .unwrap();

    let fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
    let area = Rectangle::new(Point::new(0, 0), fbuf.size());
    hardware
        .display
        .lock()
        .unwrap()
        .fill_contiguous(&area, render.data)
        .unwrap();
}

pub fn buttons(mut render: ResMut<Render>, hardware: ResMut<Hardware>) {
    render.button_a_pressed = hardware.button_a.is_low();
    render.button_b_pressed = hardware.button_b.is_low();
    render.button_x_pressed = hardware.button_x.is_low();
    render.button_y_pressed = hardware.button_y.is_low();
}

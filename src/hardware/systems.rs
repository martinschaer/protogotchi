use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;

use super::resources::{Hardware, UIConfig};
use crate::{
    menu::resources::GameState, COLOR_BLUE, COLOR_LIGHT_BLUE, COLOR_PURPLE, H_SIZE, W_SIZE,
};

pub fn startup(mut commands: Commands, mut hardware: ResMut<Hardware>) {
    commands.insert_resource(UIConfig {
        character_style: MonoTextStyle::new(&FONT_6X10, COLOR_LIGHT_BLUE),
    });

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

pub fn render_loop(
    time: Res<Time>,
    mut hardware: ResMut<Hardware>,
    ui_config: ResMut<UIConfig>,
    game_state: Res<GameState>,
) {
    // FPS
    // let mut fps = 0_u8;
    // let mut last_time = std::time::Instant::now();
    // let mut now: Instant;
    // let tick = std::time::Duration::from_millis(250);
    // let mut count = 0_u8;
    // fps += 1;
    // now = std::time::Instant::now();
    let elapsed = time.elapsed_seconds_f64();

    // led
    let y = (elapsed.sin() + 1.) * 0.5;
    let stepped_y = (y * 100.).round();
    hardware
        .led_r
        .set_pwm_frequency(50., stepped_y / 100.)
        .unwrap();

    // Backend for the buffer
    let button_a_is_pressed = hardware.button_a.is_low();
    let mut data = [if button_a_is_pressed {
        COLOR_PURPLE
    } else {
        COLOR_BLUE
    }; W_SIZE * H_SIZE];
    let mut fbuf = FrameBuf::new(&mut data, W_SIZE, H_SIZE);

    // Commodore 64 boot screen
    let print_text: String;
    if elapsed % 0.5 < 0.25 {
        print_text = format!("{}█", &game_state.text);
    } else {
        print_text = game_state.text.to_string();
    }
    Text::new(&print_text, Point::new(6, 10), ui_config.character_style)
        .draw(&mut fbuf)
        .unwrap();

    // Write it all to the display
    let area = Rectangle::new(Point::new(0, 0), fbuf.size());
    hardware
        .display
        .lock()
        .unwrap()
        .fill_contiguous(&area, data)
        .unwrap();
}

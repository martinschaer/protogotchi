use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
// use embedded_graphics_simulator::SimulatorDisplay;

use crate::{menu::resources::GameState, Sim};

pub struct SimulatorPlugin;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
// palette
//
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

// #[derive(Resource)]
// struct SimulatorState {
//     display: &'static mut SimulatorDisplay<Rgb565>,
// }

fn startup(time: Res<Time>, game_state: Res<GameState>, mut sim: Local<Sim>) {
    let elapsed = time.elapsed_seconds_f64();

    // Backend for the buffer
    // let button_a_is_pressed = hardware.button_a.is_low();
    let button_a_is_pressed = false;
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
    Text::new(
        &print_text,
        Point::new(6, 10),
        MonoTextStyle::new(&FONT_6X10, COLOR_LIGHT_BLUE),
    )
    .draw(&mut fbuf)
    .unwrap();

    // Write it all to the display
    let area = Rectangle::new(Point::new(0, 0), fbuf.size());
    sim.display.fill_contiguous(&area, data).unwrap();
    // let output_settings = OutputSettingsBuilder::new().build();
    // Window::new("Hello World", &output_settings).show_static(&display);
}

fn render_loop() {}

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, render_loop);
    }
}

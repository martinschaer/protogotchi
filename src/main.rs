use bevy::{app::ScheduleRunnerPlugin, prelude::*};

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use mipidsi::Display;
use mipidsi::{models::ST7789, Builder};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::sync::Mutex;
// use std::thread::sleep;
use std::time::Duration;

const SPI_DC: u8 = 9;
const BACKLIGHT: u8 = 13;

const BUTTON_A: u8 = 5;
const BUTTON_B: u8 = 6;
const BUTTON_X: u8 = 16;
const BUTTON_Y: u8 = 24;

const LED_R: u8 = 17;
const LED_G: u8 = 27;
const LED_B: u8 = 22;

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

#[derive(Resource)]
struct GameState {
    text: String,
}

#[derive(Resource)]
#[allow(dead_code)]
struct Hardware {
    button_a: InputPin,
    button_b: InputPin,
    button_x: InputPin,
    button_y: InputPin,
    led_r: OutputPin,
    led_g: OutputPin,
    led_b: OutputPin,
    backlight: OutputPin,
    display: Mutex<Display<SPIInterfaceNoCS<Spi, OutputPin>, ST7789, OutputPin>>,
}
impl Default for Hardware {
    fn default() -> Self {
        let gpio = Gpio::new().unwrap();
        let dc = gpio.get(SPI_DC).unwrap().into_output();
        let mut delay = Delay::new();
        let clock_speed = 60_000_000_u32;
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, clock_speed, Mode::Mode0).unwrap();
        let di = SPIInterfaceNoCS::new(spi, dc);
        let display_mutex = Mutex::new(
            Builder::st7789(di)
                .with_display_size(H_SIZE as u16, W_SIZE as u16)
                .with_orientation(mipidsi::Orientation::LandscapeInverted(true))
                .with_invert_colors(mipidsi::ColorInversion::Inverted)
                .init(&mut delay, None::<OutputPin>)
                .unwrap(),
        );

        Hardware {
            button_a: gpio.get(BUTTON_A).unwrap().into_input_pullup(),
            button_b: gpio.get(BUTTON_B).unwrap().into_input(),
            button_x: gpio.get(BUTTON_X).unwrap().into_input(),
            button_y: gpio.get(BUTTON_Y).unwrap().into_input(),
            led_r: gpio.get(LED_R).unwrap().into_output(),
            led_g: gpio.get(LED_G).unwrap().into_output(),
            led_b: gpio.get(LED_B).unwrap().into_output(),
            backlight: gpio.get(BACKLIGHT).unwrap().into_output(),
            display: display_mutex,
        }
    }
}

#[derive(Resource)]
struct UIConfig {
    character_style: MonoTextStyle<'static, Rgb565>,
}

fn startup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut hardware: ResMut<Hardware>,
) {
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

    commands.insert_resource(UIConfig {
        character_style: MonoTextStyle::new(&FONT_6X10, COLOR_LIGHT_BLUE),
    });
    let char_w = 6_usize;
    let cols = W_SIZE / char_w;
    let line = "**** COMMODORE 64 BASIC V2 ****";
    let line_cols = line.len();
    let line_pad = (cols - line_cols) / 2;
    game_state.text = String::new();
    for _ in 0..line_pad {
        game_state.text.push(' ');
    }
    game_state.text.push_str(line);
    game_state
        .text
        .push_str("\n\n 64K RAM SYSTEM  38911 BASIC BYTES FREE\n\nREADY.\n");
}

fn render_loop(
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

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        // .init_resource::<Hardware>()
        .add_systems(Startup, startup)
        .add_systems(Update, render_loop)
        .run();
}

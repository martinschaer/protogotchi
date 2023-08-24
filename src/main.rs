use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use mipidsi::Builder;
use rppal::gpio::{Gpio, OutputPin};
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{process::ExitCode, time::Instant};

const SPI_DC: u8 = 9;
const SPI_CS: u8 = 1;
const BACKLIGHT: u8 = 13;

const BUTTON_A: u8 = 5;
// const BUTTON_B: u8 = 6;
// const BUTTON_X: u8 = 16;
// const BUTTON_Y: u8 = 24;

const LED_R: u8 = 17;
const LED_G: u8 = 27;
const LED_B: u8 = 22;

const W_SIZE: usize = 320;
const H_SIZE: usize = 240;
// const W: i32 = W_SIZE as i32;
// const H: i32 = H_SIZE as i32;

// bg
const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

fn main() -> ExitCode {
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(SPI_DC).unwrap().into_output();
    let cs = gpio.get(SPI_CS).unwrap().into_output();
    let mut backlight = gpio.get(BACKLIGHT).unwrap().into_output();

    let mut delay = Delay::new();

    let clock_speed = 60_000_000_u32;
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, clock_speed, Mode::Mode0).unwrap();
    let di = SPIInterface::new(spi, dc, cs);

    let mut display = Builder::st7789(di)
        .with_display_size(H_SIZE as u16, W_SIZE as u16)
        .with_orientation(mipidsi::Orientation::LandscapeInverted(true))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, None::<OutputPin>)
        .unwrap();

    // Buttons
    let button_a = gpio.get(BUTTON_A).unwrap().into_input_pullup();
    // let button_b = gpio.get(BUTTON_B).unwrap().into_input();
    // let button_x = gpio.get(BUTTON_X).unwrap().into_input();
    // let button_y = gpio.get(BUTTON_Y).unwrap().into_input();

    // LEDs
    let mut led_r = gpio.get(LED_R).unwrap().into_output();
    let mut led_g = gpio.get(LED_G).unwrap().into_output();
    let mut led_b = gpio.get(LED_B).unwrap().into_output();
    led_r.set_pwm_frequency(50., 1.).unwrap();
    led_g.set_pwm_frequency(50., 1.).unwrap();
    led_b.set_pwm_frequency(50., 1.).unwrap();

    // Turn on backlight
    backlight.set_low();
    sleep(Duration::from_millis(150));
    backlight.set_high();

    // Clear the display initially
    display.clear(COLOR_BLUE).unwrap();

    // Text
    let character_style = MonoTextStyle::new(&FONT_6X10, COLOR_LIGHT_BLUE);
    let char_w = 6_usize;
    let cols = W_SIZE / char_w;
    let line = "**** COMMODORE 64 BASIC V2 ****";
    let line_cols = line.len();
    let line_pad = (cols - line_cols) / 2;
    let mut text = String::new();
    for _ in 0..line_pad {
        text.push(' ');
    }
    text.push_str(line);
    text.push_str("\n\n 64K RAM SYSTEM  38911 BASIC BYTES FREE\n\nREADY.\n");

    // FPS
    let mut fps = 0_u8;
    let mut last_time = std::time::Instant::now();
    let mut now: Instant;
    let tick = std::time::Duration::from_millis(250);
    let mut count = 0_u8;

    println!("Starting main loop");

    loop {
        fps += 1;
        now = std::time::Instant::now();
        let elapsed = now - last_time;

        // led
        let y = ((std::time::UNIX_EPOCH.elapsed().unwrap().as_secs_f64()).sin() + 1.) * 0.5;
        let stepped_y = (y * 100.).round();
        led_r.set_pwm_frequency(50., stepped_y / 100.).unwrap();

        if elapsed >= tick {
            last_time = now;
            count = (count + 1) % 4;
            // FPS
            if count == 0 {
                print!("\rFPS: {} ", fps);
                std::io::stdout().flush().unwrap();
                fps = 0;
            }
        }

        // Backend for the buffer
        let button_a_is_pressed = button_a.is_low();
        let mut data = [if button_a_is_pressed {
            COLOR_PURPLE
        } else {
            COLOR_BLUE
        }; W_SIZE * H_SIZE];
        let mut fbuf = FrameBuf::new(&mut data, W_SIZE, H_SIZE);

        // Commodore 64 boot screen
        let print_text: String;
        if count % 2 == 0 {
            print_text = format!("{}█", &text);
        } else {
            print_text = text.to_string();
        }
        Text::new(&print_text, Point::new(6, 10), character_style)
            .draw(&mut fbuf)
            .unwrap();

        // Write it all to the display
        let area = Rectangle::new(Point::new(0, 0), fbuf.size());
        display.fill_contiguous(&area, data).unwrap();
    }
}

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

fn main() -> ExitCode {
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(SPI_DC).unwrap().into_output();
    let cs = gpio.get(SPI_CS).unwrap().into_output();
    let mut backlight = gpio.get(BACKLIGHT).unwrap().into_output();

    let mut delay = Delay::new();

    let clock_speed = 60_000_000_u32;
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, clock_speed, Mode::Mode0).unwrap();
    let di = SPIInterface::new(spi, dc, cs);

    const W: usize = 240;
    const H: usize = 320;
    let mut display = Builder::st7789(di)
        .with_display_size(W as u16, H as u16)
        // .with_orientation(mipidsi::Orientation::Landscape(false))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, None::<OutputPin>)
        .unwrap();

    // Turn on backlight
    backlight.set_low();
    sleep(Duration::from_millis(150));
    backlight.set_high();

    // Clear the display initially
    display.clear(Rgb565::BLUE).unwrap();

    // Text
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::new(16, 30, 27));

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
        let mut data = [Rgb565::new(9, 14, 21); W * H];
        let mut fbuf = FrameBuf::new(&mut data, W, H);

        // Commodore 64 boot screen
        let text: String;
        let prompt =
            "    **** COMMODORE 64 BASIC V2 ****\n\n 64K RAM SYSTEM  38911 BASIC BYTES FREE\n\nREADY.\n";
        if count % 2 == 0 {
            text = format!("{}█", &prompt);
        } else {
            text = prompt.to_string();
        }
        Text::new(&text, Point::new(5, 5), character_style)
            .draw(&mut fbuf)
            .unwrap();

        // Write it all to the display
        let area = Rectangle::new(Point::new(0, 0), fbuf.size());
        display.fill_contiguous(&area, data).unwrap();
    }
}

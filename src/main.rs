use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use mipidsi::Builder;
use rppal::gpio::{Gpio, OutputPin};
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const SPI_DC: u8 = 9;
const SPI_CS: u8 = 1;

fn main() -> ExitCode {
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(SPI_DC).unwrap().into_output();
    let cs = gpio.get(SPI_CS).unwrap().into_output();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 60_000_000, Mode::Mode0).unwrap();
    let di = SPIInterface::new(spi, dc, cs);
    let mut delay = Delay::new();
    let mut display = Builder::st7789(di)
        .with_display_size(320, 240)
        .init(&mut delay, None::<OutputPin>)
        .unwrap();

    // Clear the display initially
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    Rectangle::new(Point::new(0, 0), Size::new(320, 240))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    let red = Rgb565::RED;

    loop {
        // Fill the display with red
        let style = PrimitiveStyleBuilder::new().fill_color(red).build();
        Rectangle::new(Point::new(0, 0), Size::new(320, 240))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        // Wait for some time
        sleep(Duration::from_millis(500));

        // Clear the display
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        Rectangle::new(Point::new(0, 0), Size::new(320, 240))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        // Wait for some time
        sleep(Duration::from_millis(500));
    }

    // ExitCode::SUCCESS
}
